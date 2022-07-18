#include <cstdint> 
#include <random> 

static_assert (std::is_same_v<std::minstd_rand::result_type, uint32_t>); 
static_assert (std::is_same_v<std::minstd_rand0::result_type, uint32_t>); 
static_assert (std::is_same_v<std::mt19937::result_type, uint32_t>); 

extern "C" 
auto minstd_rand0(std::minstd_rand0::result_type a) {
    auto *p = new std::minstd_rand0(a); 
    return p; 
}

extern "C" 
auto minstd_rand(std::minstd_rand::result_type a) {
    auto *p = new std::minstd_rand(a); 
    return p; 
}

extern "C" 
auto next_minstd_rand0(std::minstd_rand0 *v) -> std::minstd_rand0::result_type {
    return (*v)(); 
}

extern "C" 
auto next_minstd_rand(std::minstd_rand *p) -> std::minstd_rand::result_type {
    return (*p)(); 
}

extern "C" 
void delete_minstd_rand0(std::minstd_rand0 *p) {
    delete p; 
}

extern "C" 
void delete_minstd_rand(std::minstd_rand *p) {
    delete p; 
}

extern "C" 
auto mt19937(std::mt19937::result_type a) {
    auto *p = new std::mt19937(a); 
    return p; 
}

extern "C" 
auto next_mt19937(std::mt19937 *a) -> std::mt19937::result_type {
    return (*a)(); 
}

extern "C" 
auto delete_mt19937 (std::mt19937 *p) {
    delete p; 
}