<?hh

<<__EntryPoint, __Rx>>
function bad() {
  $p1 = new stdClass();
  $p1->q = array(4 => true);
  $o = new stdClass();
  $o->p = $p1;

  $lq = 'q';
  $o->p->{$lq}[4] = false;
}
