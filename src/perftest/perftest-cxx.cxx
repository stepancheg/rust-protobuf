#include <iostream>
#include <fstream>
#include <random>
#include <sstream>
#include <chrono>

#include <google/protobuf/message.h>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/io/zero_copy_stream_impl.h>

#include <stdio.h>

#include "perftest_data.pb.h"


using namespace google::protobuf;
using namespace google::protobuf::io;


#define FAIL(msg) do { fprintf(stderr, "FAIL: %s\n", msg); exit(1); } while (0)
#define VERIFY(param) do { if (!(param)) FAIL(#param); } while(0)


template <typename TFunc>
uint64_t measure_ns(TFunc func) {
    auto start = std::chrono::system_clock::now();
    func();
    auto end = std::chrono::system_clock::now();
    return std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
}

template <typename TFunc>
void measure_and_print(const std::string& name, unsigned iter, TFunc func) {
    auto ns = measure_ns(func);
    auto ns_per_iter = ns / iter;
    printf("%s: %u ns per iter\n", name.c_str(), (unsigned) ns_per_iter);
}

struct TestRunner {
    uint32_t data_size = 1000000;

    template <typename M>
    void test(const char* name, const RepeatedPtrField<M>& data) {
        std::mt19937 rng;
        std::uniform_int_distribution<std::mt19937::result_type> dist(0, data.size() - 1);

        std::vector<M> randomData;

        auto totalSize = 0;
        while (totalSize < data_size) {
            const auto& item = data.Get(dist(rng));
            randomData.push_back(item);
            totalSize += item.ByteSize();
        }

        std::string s;
        measure_and_print(std::string() + name + " write", randomData.size(), [&] () {
            StringOutputStream ss(&s);
            CodedOutputStream os(&ss);
            for (int i = 0; i < randomData.size(); ++i) {
                auto size = randomData[i].ByteSize();
                os.WriteVarint32(size);
                randomData[i].SerializeWithCachedSizes(&os);
            }
        });

        RepeatedPtrField<M> readData;

        measure_and_print(std::string() + name + " read", randomData.size(), [&] () {
            CodedInputStream is((const uint8*) s.data(), s.size());
            while (is.BytesUntilLimit() > 0) {
                uint32 length;
                bool okLength = is.ReadVarint32(&length);
                VERIFY(okLength);
                auto oldLimit = is.PushLimit(length);
                bool okReadMsg = readData.Add()->MergeFromCodedStream(&is);
                VERIFY(okReadMsg);
                is.PopLimit(oldLimit);
            }
        });

        // TODO: compare content
        VERIFY(randomData.size() == readData.size());

        auto count = 0;
        measure_and_print(std::string() + name + " read reuse", randomData.size(), [&] () {
            M msg;
            CodedInputStream is((const uint8*) s.data(), s.size());
            while (is.BytesUntilLimit() > 0) {
                msg.Clear();
                uint32 length;
                bool okLength = is.ReadVarint32(&length);
                VERIFY(okLength);
                auto oldLimit = is.PushLimit(length);
                bool okReadMsg = msg.MergeFromCodedStream(&is);
                VERIFY(okReadMsg);
                is.PopLimit(oldLimit);
                count += 1;
            }
        });

        VERIFY(randomData.size() == count);
    }
};

int main(int argc, char* argv[]) {
    std::ifstream is("perftest_data.pbbin");
    PerftestData perftestData;
    bool ok = perftestData.ParsePartialFromIstream(&is);
    VERIFY(ok);
    TestRunner runner;

    if (argc > 1) {
        runner.data_size = atoi(argv[1]);
    }

    runner.test("test1", perftestData.test1());
    runner.test("test_repeated_bool", perftestData.test_repeated_bool());
    runner.test("test_repeated_packed_int32", perftestData.test_repeated_packed_int32());
    runner.test("test_repeated_messages", perftestData.test_repeated_messages());
    runner.test("test_optional_messages", perftestData.test_optional_messages());
    runner.test("test_strings", perftestData.test_strings());
    runner.test("test_small_bytearrays", perftestData.test_small_bytearrays());
    runner.test("test_large_bytearrays", perftestData.test_large_bytearrays());
    return 0;
}
