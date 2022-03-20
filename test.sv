// adsa
// ddd
// dasd
// aaaa

module and_op (a ,b ,c );
  output    [1   :0   ]a ;
  output    [100 :0   ]c ;
  input     [50  :0   ]b ;
  assign a =1'b1;
  assign b =a ;
  assign c =~a ;
endmodule // and_op
