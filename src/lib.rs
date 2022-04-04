use sv_parser::{Locate, RefNode, SyntaxTree,WhiteSpace};



/// SV format status
#[derive(Clone)]
pub struct FormatStatus<'b> {
    // store the formatted result
    pub buffer: String,
    tail_indent: Option<usize>, // append newline and indent in the next line
    // current_node: RefNode<'a>,
    node_locate: Locate,
    pub syntax_tree: &'b SyntaxTree,
    // insert head space and tail space before and after current Locate's str
    need_tail_delimiter: bool,
    need_head_delimiter: bool,
    // RefNode will be aborted when occuring space, 
    abort_node: bool,
    // indent level will increasement by step when occuring conditional class
    indent_level: usize,
    current_line_keep_old_indent: bool,
    // when handle port declaration, signal width related will be aligned
    handle_port_declare: bool,
    handle_symbol: bool,
    port_declare_string_len: usize,
    // when handle comment, formatting will not be applied
    handle_comment: bool,
    symbol_in_special_use: bool,
    
    
}

impl<'a, 'b> FormatStatus<'a> {
    pub fn new(syntax_tree: &'a SyntaxTree) -> Self {
        Self {
            syntax_tree,
            buffer: String::new(),
            tail_indent: None,
            node_locate: Locate {
                offset: 0usize,
                line: 1u32,
                len: 0usize,
            },
            need_tail_delimiter: false,
            need_head_delimiter: false,
            abort_node: false,
            indent_level: 0,
            current_line_keep_old_indent: false,
            handle_port_declare: false,
            handle_symbol: false,
            port_declare_string_len: 0usize,
            handle_comment: false,
            symbol_in_special_use: false,
        }
    }

    // add code here
    pub fn append<'c>(&mut self, locate: &'c Locate) {


        if !self.abort_node && !self.handle_comment {

            if self.need_head_delimiter {
                self.buffer.push_str(" ");
            }

            if locate.line != self.node_locate.line {
                self.buffer.push_str("\n");
                if self.current_line_keep_old_indent == true {
                    (0..self.indent_level - 1).for_each(|_| self.buffer.push_str("  "));
                } else {
                    (0..self.indent_level).for_each(|_| self.buffer.push_str("  "));
                }
            }
            let ongoing_str = self.syntax_tree.get_str(locate).unwrap();
            if self.handle_port_declare {
                self.port_declare_string_len += locate.len;
                if ongoing_str == "[" && self.port_declare_string_len != 10usize {
                    let padding_space = 10usize - self.port_declare_string_len;
                    (0..padding_space).for_each(|_| self.buffer.push_str(" "));
                    self.port_declare_string_len += padding_space;
                    self.buffer.push_str(ongoing_str);
                } else if ongoing_str == ":" && self.port_declare_string_len != 15usize {
                    let padding_space = 15usize - self.port_declare_string_len;
                    (0..padding_space).for_each(|_| self.buffer.push_str(" "));
                    self.port_declare_string_len += padding_space;
                    self.buffer.push_str(ongoing_str);
                } else if ongoing_str == "]" && self.port_declare_string_len != 20usize {
                    let padding_space = 20usize - self.port_declare_string_len;
                    (0..padding_space).for_each(|_| self.buffer.push_str(" "));
                    self.port_declare_string_len += padding_space;
                    self.buffer.push_str(ongoing_str);
                } else if ongoing_str == ";" {
                    self.buffer.push_str(ongoing_str);
                    self.handle_port_declare = false;
                    self.port_declare_string_len = 0;
                } else {
                    self.buffer.push_str(ongoing_str);
                }
            } else {
                self.buffer.push_str(ongoing_str);
            }
            
            if self.need_tail_delimiter {
                self.buffer.push_str(" ");
            }
        } else if self.handle_comment {
            let ongoing_str = self.syntax_tree.get_str(locate).unwrap();
            self.buffer.push_str(ongoing_str);
            self.handle_comment = false;
        }

        // reset status
        self.node_locate = locate.clone();
        // self.need_tail_newline = false;
        self.need_tail_delimiter = false;
        self.need_head_delimiter = false;
        self.tail_indent = None;
        self.current_line_keep_old_indent = false;
    }

    pub fn exec_format(&mut self) {
        for node in self.syntax_tree {
            // self.current_node = &node;
            match node {
                RefNode::WhiteSpace(x) => {
                    self.abort_node = true;
                    if let WhiteSpace::Newline(_) = x {
                        self.abort_node = true;
                        let whitespace_str = self.syntax_tree.get_str(x).unwrap();
                        let newline_c = whitespace_str.matches("\n").count();
                        self.buffer.push_str(&"\n".repeat(newline_c-1));
                    }
                }
                RefNode::Comment(_) => {
                    self.handle_comment = true;
                }
                RefNode::Symbol(x) => {
                    self.handle_symbol = true;
                    self.abort_node = false;
                    let ongoing_str = self.syntax_tree.get_str(&x.nodes.0).unwrap();
                    if !self.symbol_in_special_use {
                        if ongoing_str == "}" 
                        || ongoing_str == ")"
                        || ongoing_str == "]"
                        {
                            self.need_tail_delimiter = true;
                        } else if ongoing_str == ":" && !self.handle_port_declare {
                            self.need_tail_delimiter = false;
                        } else if ongoing_str == ";" && !self.need_tail_delimiter {
                            self.need_head_delimiter = false;                            
                        }
                    } else {
                        if ongoing_str == ";" {
                            self.symbol_in_special_use = false;
                        } else {
                            self.need_head_delimiter = false;
                            self.need_tail_delimiter = false;
                        }
                    }

                }
                RefNode::Number(_) => {
                    if self.handle_symbol {
                        self.need_head_delimiter = true;
                        self.handle_symbol = false;
                    }
                }
                RefNode::Locate(x) => {
                    self.append(x);
                }
                RefNode::PackageImportItemIdentifier(_) => {
                    self.need_tail_delimiter = false;
                    self.symbol_in_special_use = true;
                }
                RefNode::SimpleIdentifier(x) => {
                    let _ongoing_str = self.syntax_tree.get_str(&x.nodes.0).unwrap();
                    self.need_tail_delimiter = true;
                    // symbol before indentifier
                    if self.handle_symbol {
                        self.need_head_delimiter = true;
                        self.handle_symbol = false;
                    }

                    if self.symbol_in_special_use {
                        self.need_tail_delimiter = false;
                    }
                }
                RefNode::Keyword(x) => {
                    let ongoing_str = self.syntax_tree.get_str(&x.nodes.0).unwrap();

                    if ongoing_str == "begin"
                        || ongoing_str == "module"
                        || ongoing_str == "program"
                        || ongoing_str == "class"
                        || ongoing_str == "function"
                        || ongoing_str == "package"
                    {
                        self.indent_level += 1;
                        self.current_line_keep_old_indent = true;
                        // if ongoing_str == "begin" {
                        //     self.tail_indent = Some(self.indent_level);
                        // }
                    } else if ongoing_str == "end"
                        || ongoing_str == "endmodule"
                        || ongoing_str == "endclocking"
                        || ongoing_str == "endcase"
                        || ongoing_str == "endgroup"
                        || ongoing_str == "endclass"
                        || ongoing_str == "endfunction"
                        || ongoing_str == "endpackage"
                    {
                        self.indent_level -= 1;
                        self.current_line_keep_old_indent = false;
                    } else if ongoing_str == "input"
                        || ongoing_str == "output"
                        || ongoing_str == "inout"
                    {
                        self.handle_port_declare = true;
                    }

                    self.abort_node = false;
                    self.need_tail_delimiter = true;
                    // self.need_tail_newline = false;
                    if self.handle_symbol {
                        self.need_head_delimiter = true;
                        self.handle_symbol = false;
                    }
                }

                _ => {
                    self.abort_node = false;
                }
            }
        }
    }
}
