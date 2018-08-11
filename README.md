# nukleus
##### the future of operating systems.

nukleus is a small, container* based OS writeen in Rust. Instead of users running a desktop GUI, users have a terminal that allows them to interact with and create containers* that are each running one or multiple tasks.

A container can have one of three forms:

 - An x86 emulator running a set OS or .asm/.s file
 - A program (or multiple programs) running on a set number of threads
 - A normal desktop GUI with programs on the threads that haven't neen allocated to a task yet

## goals

 - Uncomprimising speed - lightspeed is too slow!
 - Itsy-bitsy - we want your files to take up space on your hard drive, not the OS. As of 8-10-18, with documentation, we are 158730.159 times smaller than Windows 10
 - Suitable for older hardware - breathe new life into your old PC
 - Avoiding feature-bloat - we let the user decide what he/she needs
