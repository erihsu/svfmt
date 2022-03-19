module and_op (a,b,c);
	output a;
	output c;
	input b;

	assign a = 1'b1;
	assign b = a;

	assign c = ~ a;

endmodule // and_op


