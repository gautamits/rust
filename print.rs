//this file shows ways of formatted printing
fn main(){
	println!("{} days",33);
	//positional parameters
	println!("{0} this {1} example {0} shows use of optional parameters","hello","awesome");
	//mapping
	println!("{subject} {verb} {object}",subject="ram",verb="plays",object="hockey");
	//special formatting can be specified by using :
	println!("{} of {:b} people know binary",1,2);
	////right alignment
	println!("{number:>width$}",number=1,width=6);
	//padding
	println!("{number:>0width$}",number=1,width=6);
	//mising argument
	println!("my name is {1}, {0} {1}","James","Bond");
	


}