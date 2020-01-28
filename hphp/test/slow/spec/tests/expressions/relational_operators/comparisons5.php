<?hh

/*
   +-------------------------------------------------------------+
   | Copyright (c) 2015 Facebook, Inc. (http://www.facebook.com) |
   +-------------------------------------------------------------+
*/
<<__EntryPoint>> function main(): void {
error_reporting(0);

///*
// Two array types

$oper1 = varray[[], varray[10,20], darray["red"=>0,"green"=>0]];
$oper2 = varray[[], varray[10,20], varray[10,20,30], varray[10,30], darray["red"=>0,"green"=>0], darray["green"=>0,"red"=>0]];

foreach ($oper1 as $e1)
{
    foreach ($oper2 as $e2)
    {
        echo "{$e1} >        {$e2}  result: "; var_dump($e1 > $e2);
        echo "{$e2} <=       {$e1}  result: "; var_dump($e2 <= $e1);
        echo "---\n";
        echo "{$e1} >=       {$e2}  result: "; var_dump($e1 >= $e2);
        echo "{$e2} <        {$e1}  result: "; var_dump($e2 < $e1);
        echo "---\n";
        echo "{$e1} <        {$e2}  result: "; var_dump($e1 < $e2);
        echo "{$e2} >=       {$e1}  result: "; var_dump($e2 >= $e1);
        echo "---\n";
        echo "{$e1} <=       {$e2}  result: "; var_dump($e1 <= $e2);
        echo "{$e2} >        {$e1}  result: "; var_dump($e2 > $e1);
        echo "=======\n";
    }
    echo "-------------------------------------\n";
}
//*/
}
