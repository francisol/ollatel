import os
import sys
os.chdir(sys.argv[1])
p1= os.path.abspath(os.path.join(sys.argv[1],"../python"))
p2=os.path.abspath(os.path.join(sys.argv[1],"../python/Scripts"))
os.putenv('PATH',f"{p1};{p2};{os.getenv('PATH')}")
os.system("init-ollama.bat")

