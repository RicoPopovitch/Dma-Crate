#include <string>
#include <DMALibrary/Memory/Memory.h>

extern "C" bool c_init(const char* process_name) {
    std::string proc_name(process_name);
    return mem.Init(proc_name, false, false);
}

extern "C" uint64_t c_get_base_address(const char* module_name) {
    std::string mod_name(module_name);
    return mem.GetBaseDaddy(mod_name);
}

extern "C" uint64_t c_get_base_size(const char* module_name) {
    std::string mod_name(module_name);
    return mem.GetBaseSize(mod_name);
} 

extern "C" const char* c_get_module_list(const char* process_name) {
    static std::string module_list_str;
    std::string proc_name(process_name);
    auto modules = mem.GetModuleList(proc_name);
    
    module_list_str.clear();
    for (const auto& module : modules) {
        module_list_str += module + '\0';
    }
    module_list_str += '\0';
    
    return module_list_str.c_str();
}

extern "C" bool c_read(uint64_t address, void* buffer, size_t size) {
    return mem.Read(address, buffer, size);
}

extern "C" bool c_write(uint64_t address, void* buffer, size_t size) {
    return mem.Write(address, buffer, size);
}

extern "C" {

typedef struct {
    uintptr_t vaStart;
    uintptr_t vaEnd;
} HeapRegionC;

bool c_get_heap_regions(HeapRegionC* outHeaps, size_t maxHeaps, size_t* outCount) {
    std::vector<Memory::HeapRegion> heaps;
    bool ok = mem.GetHeapRegions(heaps);
    if (!ok) {
        if (outCount) *outCount = 0;
        return false;
    }

    const size_t count = heaps.size();
    if (outCount) *outCount = count;

    if (outHeaps && maxHeaps >= count) {
        for (size_t i = 0; i < count; ++i) {
            outHeaps[i].vaStart = heaps[i].start;
            outHeaps[i].vaEnd   = heaps[i].end;
        }
    }
    return true;
}

} // extern "C"