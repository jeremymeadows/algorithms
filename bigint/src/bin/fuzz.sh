#!/usr/bin/env bash

if [ -n $1 ]; then
    iterations=$1
else
    iterations=10
fi

echo $1 iterations

while [ $iterations -gt 0 ]; do
    len=$(($RANDOM % 16 + 1))
    while [ $len -gt 0 ]; do
        a="$a$RANDOM"
        len=$(($len - 1))
    done

    len=$(($RANDOM % 12 + 1))
    while [ $len -gt 0 ]; do
        b="$b$RANDOM"
        len=$(($len - 1))
    done

    len=$(($RANDOM % 4 + 1))
    while [ $len -gt 0 ]; do
        c="$c$RANDOM"
        len=$(($len - 1))
    done

    echo ''
    echo $a
    echo $b
    echo $c

    py=$(python << EOF
print(format($a + $b, 'x'))
print(format($a - $b, 'x'))
print(format($a * $b, 'x'))
print(format($a // $b, 'x'))
#print(format($a - ($a // $b), 'x'))
print(format($a % $b, 'x'))
#print(format(pow($a, $c, $b)))
EOF
)

    rs=$(cargo run --release --bin test 2>/dev/null <<< "$a $b $c")

    if [ "$rs" == "$py" ] ; then
        echo passed
    else
        echo failed
        printf "python:\n$py\n"
        printf "rust:\n$rs\n"
        exit
    fi

    iterations=$(($iterations - 1))
done