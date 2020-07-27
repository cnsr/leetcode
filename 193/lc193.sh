while IFS= read -r line
do
  [[ $line =~ (^\([0-9]{3}\) [0-9]{3}-[0-9]{4})$|^[0-9]{3}-([0-9]{3})-[0-9]{4}$ ]] && echo "$line"
done < "file.txt"
