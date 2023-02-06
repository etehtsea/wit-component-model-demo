#include "demo.h"

void exports_run() {
    demo_string_t my_string;
    demo_string_set(&my_string, "Hello from the guest (C)!");

    imports_print(&my_string);
}
