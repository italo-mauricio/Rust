
static _Y: u32 = 13;


fn main() {
    let x = 5;
    let z = true;
    let numbers = [1, 2, 3];

    let users = get_users();
}


/* ---------------------------------------------- MEMORY ----------------------------------------------------------

            Content                      Size                 Lifetime                     Cleaneup

Static: - Program Binary                 Fixed              Whole program              when program terminates
        - Static variables
        - String literals
        - knwon compilation

Stack:  - Function arguments             Dynamic                Function                When function returns
        - Local variables                Top limit
        - Each thread has an 
          isolated stack
        - Knwon compilation size


Heap:   - Values that live beyond       Dynamic                Defined by              Manually or via GC or via RAII 
          functions                     Up to computer         programmer or
        - Shared across threads         limit                  language
        - Large values
        - Dynamic size values
        - Unknown compilation size



FREE











*/