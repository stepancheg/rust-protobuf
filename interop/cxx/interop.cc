#include <iostream>
#include <vector>

#include <google/protobuf/util/json_util.h>

#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>

#include "interop_pb.pb.h"

using namespace std;

using namespace google::protobuf::util;


void usage(const char* argv0) {
    cerr << "usage: " << argv0 << " <subcommand>\n";
}

vector<char> read_stdin_to_end() {
    vector<char> r;
    size_t size = 0;
    for (;;) {
        r.resize(size * 2 + 100);
        auto re = read(STDIN_FILENO, r.data() + size, r.size() - size);
        if (re == 0) {
            r.resize(size);
            return r;
        }
        if (re < 0) {
            perror("read");
            _exit(1);
        }
        size += re;
    }
}

int main(int argc, const char** argv) {
    if (argc <= 1) {
        usage(argv[0]);
        _exit(1);
    }

    if (!strcmp(argv[1], "self-test")) {
        cerr << "interop OK\n";
        return 0;
    }

    if (!strcmp(argv[1], "json-encode")) {
        InteropMessageList m;

        auto parse_ok = m.ParseFromFileDescriptor(STDIN_FILENO);
        if (!parse_ok) {
            cerr << "failed to ParseFromFileDescriptor\n";
            _exit(21);
        }

        string json_output;
        auto json_ok = MessageToJsonString(m, &json_output);
        if (!json_ok.ok()) {
            cerr << "failed to MessageToJsonString\n";
            _exit(22);
        }

        cout << json_output << "\n";
        return 0;
    }

    if (!strcmp(argv[1], "json-decode")) {
        InteropMessageList m;

        auto json_text_vec = read_stdin_to_end();
        string json_text(json_text_vec.data(), json_text_vec.size());
        if (json_text.empty()) {
            cerr << "empty\n";
            _exit(31);
        }
        auto json_ok = JsonStringToMessage(json_text, &m);
        if (!json_ok.ok()) {
            cerr << "failed to JsonStringToMessage: " << json_ok.error_message() << "\n";
            _exit(32);
        }

        auto message_ok = m.SerializeToFileDescriptor(STDOUT_FILENO);
        if (!message_ok) {
            cerr << "failed to SerializeToFileDescriptor\n";
            _exit(33);
        }

        return 0;
    }

    usage(argv[0]);
    return 1;
}

// vim: set ts=4 sw=4 et:
