RESULTS=()



echo -e "\033[93m"
echo "    pretty pretty = pretty"
echo "    ======================"
echo -e -n "\033[0m"

COUNT=0
COUNT_PASSED=0
for PROGRAM in `find programs/1-scan+parse/valid -name "*.go" `
do
  ((COUNT++))

  if ( ./run.sh pretty $PROGRAM > tmp1 2>/dev/null &&
  ./run.sh pretty tmp1 > tmp2 &&
  diff tmp1 tmp2 )
  then
    ((COUNT_PASSED++))
  else
    STATUS_COLOUR="31"
    echo "$PROGRAM " | tr -d '\n'
    echo -e -n " \033[0;${STATUS_COLOUR}m[fail]\033[0m"
    echo

    #if [ $LOG -eq 1 ]
    #then
      #echo "$TEST: $OUTPUT [fail]" >> ${PHASE_NAME}_${TYPE}.log
    #fi
  fi

done
if [ $COUNT -eq $COUNT_PASSED ]
then
  STATUS_COLOUR="42"
else
  STATUS_COLOUR="41"
fi
echo -e "\e[${STATUS_COLOUR}m# pretty pretty = pretty: ${COUNT_PASSED}/${COUNT}\e[49m"
RESULTS+=("\e[${STATUS_COLOUR}m# pretty pretty = pretty: ${COUNT_PASSED}/${COUNT}\e[49m")




echo -e "\033[93m"
echo "    pretty programs valid"
echo "    ======================"
echo -e -n "\033[0m"
COUNT=0
COUNT_PASSED=0
if [[ -x ~cs520/golitec ]] 
then
  for PROGRAM in `find programs/1-scan+parse/valid -name "*.go" `
  do
    ((COUNT++))

    if ( ./run.sh pretty $PROGRAM > tmp1 2>/dev/null &&
      ~cs520/golitec parse < tmp1 >/dev/null) then
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
  echo -e "\e[${STATUS_COLOUR}m# pretty programs valid: ${COUNT_PASSED}/${COUNT}\e[49m"
  RESULTS+=("\e[${STATUS_COLOUR}m# pretty programs valid: ${COUNT_PASSED}/${COUNT}\e[49m")
else 

  STATUS_COLOUR="43"
  echo -e "\e[${STATUS_COLOUR}m# pretty programs valid: Can't find ~cs520/golitec\e[49m"

fi


echo RESULTS:
for i in ${!RESULTS[*]}; do
	echo -e ${RESULTS[$i]}
done
