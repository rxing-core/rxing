IN_DIR=$1
OUT_FILE=$2
COUNT=0
echo "Writing from '${IN_DIR}' to '${OUT_FILE}'"
for fn in $(ls -p ${IN_DIR} | grep -v /)
do
    echo "Found file ${IN_DIR}/${fn}"
    echo "Adding contents of ${fn} to ${OUT_FILE}"
    ECHO "// NEW FILE: ${fn}" >> ${OUT_FILE}
    cat ${IN_DIR}/${fn} >> ${OUT_FILE}
    COUNT=$((COUNT + 1))
done;
echo "Done compiling ${COUNT} files from ${IN_DIR} to ${OUT_FILE}"