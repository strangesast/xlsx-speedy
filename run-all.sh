for d in */ ; do
  name=${d%/}
  bash ./run.sh $name
done
