set cgwin-path=C:\cygwin64\bin\bash
set working-dir=%cd%
start %cgwin-path% --login -i -c "cd ""%working-dir%\SDL2-2.0.14""; ./configure; make; make install; read"