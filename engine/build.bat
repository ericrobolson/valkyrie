set cgwin-path=C:\cygwin64\bin\bash
set working-dir=%cd%
start %cgwin-path% --login -i -c "cd ""%working-dir%""; make; echo ""press any to exit""; read"