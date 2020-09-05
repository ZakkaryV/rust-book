use company::{Company, Operations};
use std::io::stdin;

mod company {
    use mylib::util::selection_sort_str;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum Operations {
        CreateDpmt,
        RetreiveDpmt,
        AddEmployee,
        None,
    }

    #[derive(Debug)]
    pub struct Company {
        departments: HashMap<String, Vec<String>>,
        pub action_header: Option<String>,
        pub name: Option<String>,
        pub operation: Operations,
    }

    impl Company {
        pub fn new() -> Self {
            let mut departments: HashMap<String, Vec<String>> = HashMap::new();

            Self {
                action_header: None,
                departments,
                name: None,
                operation: Operations::None,
            }
        }
        pub fn set_name(&mut self, name: String) {
            self.name = Some(name);
        }
        pub fn set_action_header(&mut self, header: Option<String>) {
            if let Some(h) = header {
                self.action_header = Some(h);
            } else {
                self.action_header = None;
            }
        }
        pub fn get_action_header(&self) -> &Option<String> {
            &self.action_header
        }
        pub fn name(&self) -> Option<&String> {
            if let Some(name) = &self.name {
                Some(name)
            } else {
                None
            }
        }
        pub fn create_dpmt(&mut self, dpmt_name: String) -> Result<String, String> {
            match &self.departments.get(&dpmt_name) {
                Some(_) => Err(dpmt_name.to_owned()),
                None => {
                    &self.departments.insert(dpmt_name.clone(), Vec::new());
                    Ok(dpmt_name.to_owned())
                }
            }
        }
        pub fn get_dpmts(&mut self) -> Option<Vec<(String, i32)>> {
            if self.departments.len() > 0 {
                let mut with_num_employees: Vec<(String, i32)> = Vec::new();
                for (d, v) in &self.departments {
                    with_num_employees.push((d.to_owned(), v.len() as i32))
                }

                Some(with_num_employees)
            } else {
                None
            }
        }
        pub fn set_operation(&mut self, op: Operations) {
            self.operation = op;
        }
        pub fn add_employees(&mut self, dpmt_name: String, emp_names: String) {
            let list = self.departments.get_mut(&dpmt_name);
            if let Some(vec) = list {
                for name in emp_names.split(",") {
                    vec.push(name.trim().to_owned());
                }
            }
        }
        pub fn get_sorted(&mut self, department: String) -> Option<Vec<String>> {
            let list = self.departments.get_mut(&department);
            if let Some(vec) = list {
                // Some(selection_sort_str(&mut vec[..]));
                vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                Some(vec.clone())
            } else {
                None
            }
        }
    }
}

fn print_header(header: String) {
    print!("{}[2J", 27 as char);
    println!("----------------------");
    println!(" COMPANY BUILDER 1.0");
    println!("----------------------");
    if header.len() > 0 {
        println!("\n{}", header);
    }
}

fn set_company_name(company: &mut Company) {
    let mut name = String::new();

    print_header("Enter a name for your new company profile:\n".to_string());

    match stdin().read_line(&mut name) {
        Result::Ok(s) => println!("Success! {}", s),
        Result::Err(e) => println!("A grave mistake was made: {}", e),
    }

    company.set_name(name.trim().to_owned());
    company.set_operation(Operations::None);
}

fn choose_option(company: &mut Company) {
    let mut op = String::new();

    println!("\nChoose an operation to perform:\n");
    println!("1) Create a department");
    println!("2) Add employee to department");
    println!("3) Retrieve employee directory\n");

    stdin().read_line(&mut op).expect("Could not read!");

    match op.trim().parse() {
        Ok(n) => match n {
            1 => company.set_operation(Operations::CreateDpmt),
            2 => company.set_operation(Operations::AddEmployee),
            3 => company.set_operation(Operations::RetreiveDpmt),
            _ => {
                print_header("Choose a valid option!".to_string());
            }
        },
        Err(_) => print_header("Enter a number.\n".to_string()),
    }
}

fn create_dpmt(company: &mut Company) {
    if let Some(header) = company.get_action_header() {
        print_header(format!("{}", header));
        match company.get_dpmts() {
            Some(departments) => {
                let (table, indexed_departments) = build_table_and_index_departments(&departments);
                println!("{}", table);
            }
            None => {}
        }
        println!("Enter another department name or \"back\" to go back to the menu.\n");
    } else {
        print_header("New department name:\n".to_string());
    }

    let mut buffer = String::new();

    stdin().read_line(&mut buffer).expect("enter somethingg");

    if buffer.trim().eq("back") {
        print_header("".to_string());
        company.set_action_header(None);
        company.set_operation(Operations::None);
    } else {
        match company.create_dpmt(buffer.trim().to_owned()) {
            Ok(name) => {
                company
                    .set_action_header(Some(format!("Succesfully created {} department!\n", name)));
            }
            Err(name) => {
                company
                    .set_action_header(Some(format!("{} is taken! Try a different name.\n", name)));
            }
        };
    }
}

fn get_spaces(amt: i32) -> String {
    let mut s = String::new();
    for _i in 0..amt {
        s.push_str(" ");
    }
    s
}

fn build_table_and_index_departments(departments: &Vec<(String, i32)>) -> (String, Vec<&String>) {
    let mut table_str = String::new();
    let mut row_str: String;
    let mut longest_name_len: i32 = 4;
    let mut indexed_departments = vec![];

    // figure out the longest department name so the "name" column can have a consistent width
    for (d, _i) in departments.iter() {
        let name_len = d.len() as i32;

        if name_len > longest_name_len {
            longest_name_len = name_len;
        }
    }

    // apply top row
    let top_row: String = format!(
        "Dpt # | Name{} | Employee Count",
        get_spaces(longest_name_len - 4)
    );

    // apply table rows
    for (i, (d, n)) in departments.iter().enumerate() {
        row_str = format!(
            "{}     | {}{} | {}\n",
            i + 1,
            d,
            get_spaces(longest_name_len - d.len() as i32),
            n
        );
        table_str.push_str(&row_str);
        indexed_departments.push(d);
    }

    // give back the table and a vector with index references for selection
    (
        format!("{}\n{}\n{}", top_row, "-".repeat(top_row.len()), table_str),
        indexed_departments,
    )
}

fn add_employee(company: &mut Company) {
    match company.get_dpmts() {
        Some(departments) => {
            let mut selected_department_index = String::new();
            let (table, indexed_departments) = build_table_and_index_departments(&departments);
            if let Some(header) = company.get_action_header() {
                print_header(format!(
                    "{}\n{}\nChoose another department to add more employees or type \"back\" to return.\n",
                    header, table
                ));
            } else {
                print_header(format!("{}\nChoose a department #:\n", table));
            }

            stdin()
                .read_line(&mut selected_department_index)
                .expect("Index of the department of which to add the employees");

            if selected_department_index.trim().eq("back") {
                print_header("".to_string());
                company.set_action_header(None);
                company.set_operation(Operations::None);
            } else {
                match selected_department_index.trim().parse::<i32>() {
                    Ok(ref n) => {
                        if *n <= indexed_departments.len() as i32 {
                            let mut emp_name = String::new();
                            let dpmt_name = indexed_departments[(n - 1) as usize].to_owned();

                            print_header(format!("Enter {} employee name(s):\n", dpmt_name));

                            stdin()
                                .read_line(&mut emp_name)
                                .expect("A comma separated list of names");

                            company.set_action_header(Some(format!(
                                "Added {} to {}!\n",
                                emp_name.to_owned().trim(),
                                dpmt_name
                            )));

                            company.add_employees(dpmt_name, emp_name.trim().to_owned());
                        } else {
                            company.set_action_header(Some(
                                "Enter a valid department #\n".to_string(),
                            ));
                        }
                    }
                    Err(_e) => {
                        company.set_action_header(Some("Enter a valid department #\n".to_string()));
                    }
                }
            }
        }
        None => {
            print_header("There are no departments. Add one first!".to_string());
            company.set_operation(Operations::None);
        }
    };
}

fn retreive_sorted(company: &mut Company, last_run_results: Option<String>) {
    match company.get_dpmts() {
        Some(departments) => {
            let mut selected_department_index = String::new();
            let (table, indexed_departments) = build_table_and_index_departments(&departments);

            if let Some(r) = company.get_action_header() {
                print_header(format!(
                    "{}\n{}\nSelect another department or \"back\" to return.\n",
                    r, table
                ));
            } else {
                print_header(format!("{}\nChoose a department:\n", table));
            }

            stdin()
                .read_line(&mut selected_department_index)
                .expect("Index of department whose employees to sort");

            match selected_department_index.trim().parse::<i32>() {
                Ok(ref n) => {
                    if *n <= indexed_departments.len() as i32 {
                        let mut department_index = String::new();
                        let department_name = indexed_departments[(n - 1) as usize].to_owned();

                        // y is there no Array.join
                        let mut sorted_str = String::from(format!(
                            "Employees of {}\n-------------{}\n",
                            department_name,
                            "-".repeat(department_name.len())
                        ));
                        let mut sorted_list = company.get_sorted(department_name).unwrap();

                        let sorted_list_len = sorted_list.len();

                        if sorted_list_len > 0 {
                            sorted_list.iter_mut().enumerate().for_each(|(i, n)| {
                                let fmt_str = format!("{}) {}\n", i + 1, n.trim());
                                sorted_str.push_str(&fmt_str);
                            });
                        } else {
                            sorted_str.push_str("None :(\n");
                        }

                        company.set_action_header(Some(sorted_str));
                    }
                }
                Err(_e) => {
                    print_header("".to_string());
                    company.set_action_header(None);
                    company.set_operation(Operations::None);
                }
            }
        }
        None => {
            print_header("There are no departments. Add one first!".to_string());
            company.set_operation(Operations::None);
        }
    };
}

fn main() {
    let mut company = Company::new();
    let mut first_time: bool = true;

    loop {
        if let Some(name) = &mut company.name {
            if first_time {
                first_time = false;
                print_header(format!("Welcome to {}!", name));
            }
            match company.operation {
                Operations::CreateDpmt => create_dpmt(&mut company),
                Operations::AddEmployee => add_employee(&mut company),
                Operations::RetreiveDpmt => retreive_sorted(&mut company, None),
                _ => choose_option(&mut company),
            }
        } else {
            set_company_name(&mut company);
        }
    }
}
