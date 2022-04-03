// it's for test

`ifndef ABCD
`define ABCD

package my_package;
	import uvm_pkg::*;
	import internal_pkg::*;

	// include
	`include "testcase1.sv"
	`include "testcase2.sv"

	// end of include

	
endpackage : my_package


`endif