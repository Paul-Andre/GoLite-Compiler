RESULTS=()



echo -e "\033[93m"
echo "    autocheck if typecheck programs match reference"
echo "    ======================"
echo -e -n "\033[0m"
COUNT=0
COUNT_PASSED=0
if [[ -x ~cs520/golitec ]] 
then
  for PROGRAM in `find programs/2-typecheck/valid -name "*.go" `
  do
    ((COUNT++))

    if ( ./run.sh codegen $PROGRAM > tmp1.js 2>/dev/null && node tmp1.js > tmp1.out &&
      ~cs520/golitec codegen < $PROGRAM > tmp2.cpp 2>/dev/null && g++ -std=c++11 tmp2.cpp && ./a.out > tmp2.out &&
      diff tmp1.out tmp2.out) then
    ((COUNT_PASSED++))
    else
      STATUS_COLOUR="31"
      echo "$PROGRAM " | tr -d '\n'
      echo -e -n " \033[0;${STATUS_COLOUR}m[fail]\033[0m"
      echo
    fi

  done
  if [ $COUNT -eq $COUNT_PASSED ]
  then STATUS_COLOUR="42"
  else
    STATUS_COLOUR="41"
  fi
  echo -e "\e[${STATUS_COLOUR}m# autocheck 2-typecheck: ${COUNT_PASSED}/${COUNT}\e[49m"
  RESULTS+=("\e[${STATUS_COLOUR}m# autocheck 2-typecheck: ${COUNT_PASSED}/${COUNT}\e[49m")
else 

  STATUS_COLOUR="43"
  echo -e "\e[${STATUS_COLOUR}m# obfuscated programs valid: Can't find ~cs520/golitec\e[49m"

fi


echo RESULTS:
for i in ${!RESULTS[*]}; do
	echo -e ${RESULTS[$i]}
done
