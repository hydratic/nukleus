# A small python module designed to send code bits off to the test kernel
import subprocess

class Testing:
    def get_io():
    
    
    def test(testing, io):
        if testing == 1:
            if io == "memory":
        
            if io == "vga":
        
            if io == "ext4":
        
            if io == "container":
        
            if io == "all":
        
    def build_on_boot(binary):
    
def main():
    build_on_boot = 0
    testing = 1
    
    # which should take priority?
    precendence = "testing"
    
    if build_on_boot == 1:
    
    if testing == 1:
        if precendence == "testing":
            get_io()
        elif precendence == "build_on_boot":
            build_on_boot()
            get_io()
    elif testing == 0:
        if build_on_boot == 1:
            build_on_boot()
            
main()
