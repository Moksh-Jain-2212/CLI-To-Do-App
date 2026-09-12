use std::io; 

#[derive(Debug)]
struct Task {
    name_of_task: String,
    task_id: u32,
    is_completed: bool,
}

impl Task {
    pub fn mark_task_as_completed(&mut self) {
        self.is_completed = true;
    }

    pub fn show_whether_the_task_is_pending_or_completed(&mut self) {
        println!("{}", self.is_completed);
    }
}

fn take_input() -> usize {
    let mut taskId = String::new();

    io::stdin()
            .read_line(&mut taskId)
            .expect("Failed to take taskId as input");

    let taskID:usize = taskId.trim().parse().expect("Please enter a number");

    taskID
}

fn display_all_features(task_list: &mut Vec<Task>) {

    let features: Vec<String> = vec![
        "add_a_new_task".to_string(),
        "view_all_tasks".to_string(),
        "mark_a_test_as_completed".to_string(),
        "delete_a_task".to_string(),
        "exit_the_application".to_string(),
        "show_whether_each_task_is_pending_or_completed".to_string(),
    ];

    println!("Enter the feature you want to see");

    let si = features.len();

    for idx in 0..si {
        println!("{} for {}", idx, features[idx]);
    }

    while true  
    {
        let mut choice: u32 = take_input().try_into().unwrap();

        if choice == 0 {
            add_a_new_task(task_list);
            break;
        }
        else if choice == 1  {
            view_all_tasks(task_list);
            break;
        }
        else if choice == 2 {
            mark_a_task_as_completed(task_list);
            break;
        }
        else if choice == 3  {
            delete_a_task(task_list);
            break;
        }
        else if choice == 4 {
            exit_the_application();
            break;
        }
        else if choice == 5  {
            show_whether_each_task_is_pending_or_completed(task_list);
            break;
        }
        else {
            println!("Enter a valid number");
        }
    }

}

fn add_a_new_task(task_list: &mut Vec<Task>) {

    println!("Enter the name of the task you want to add");

    let mut task_name = String::new();

    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to take input");

    task_name = task_name.trim().to_string();  

    task_list.push(
        Task {
            name_of_task: task_name,
            task_id: (task_list.len()) as u32,
            is_completed: false,
        }
    );

    println!("Task Successfully added");

    view_all_tasks(task_list);
}

fn view_all_tasks(task_list: &mut Vec<Task>) {

    for task in task_list.iter() {
        println!("{:?}", task);
    }
    
    display_all_features(task_list);
}

fn mark_a_task_as_completed(task_list: &mut Vec<Task>) {
    println!("Enter the task_id");

    while true {    
        let taskID = take_input();

        if taskID >= ((task_list.len() as u32)).try_into().unwrap() {
            continue;
        }

        task_list[taskID].mark_task_as_completed();
        
        break;
    }

    display_all_features(task_list);
}

fn delete_a_task(task_list: &mut Vec<Task>) {
    println!("Enter the task_id you want to delete");

    while true {    
        let taskID = take_input();

        if taskID >= ((task_list.len() as u32)).try_into().unwrap() {
            continue;
        }

        task_list.remove(taskID);
        
        break;
    }

    display_all_features(task_list);
}

fn exit_the_application() {
    panic!("Exit the application");
}

fn show_whether_each_task_is_pending_or_completed(task_list: &mut Vec<Task>) {

    println!("Enter the task_id you want to see");

    while true {    
        let taskID = take_input();

        if taskID >= ((task_list.len() as u32)).try_into().unwrap() {
            continue;
        }

        task_list[taskID].show_whether_the_task_is_pending_or_completed();
        
        break;
    }

    display_all_features(task_list);
}

fn main() {

    let mut task_list: Vec<Task> = Vec::new();

    display_all_features(&mut task_list);

}
