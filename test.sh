v1="version"
v1=`grep -m 1 $v1 Cargo.toml`
v1=${v1%\"*}
echo $v1
output=${v1#*\"}
echo $output
