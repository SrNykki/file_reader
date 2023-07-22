use std::io;
use std::fs;

fn main() {
	
	println!("Digite o nome do arquivo :");
	let mut input = String::new();
	io::stdin()
	.read_line(&mut input)
	.expect("Erro");
	
	input.pop();
	
	let content = fs::read_to_string(input);
	
	if let Ok(content) = content{
    		println!("{content}");

	}else {
		println!("Arquivo não encontrado.");

	}
}

