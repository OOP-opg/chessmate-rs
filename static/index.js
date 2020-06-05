let choice = null
let login_id = null

const chose_black = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "black";
    placeholder.textContent = choice;
    console.log(choice);
}

const chose_white = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "white";
    placeholder.textContent = choice;
    console.log(choice);
}

const find_pair = () => {
    let result_placeholder = document.getElementById("result");
    fetch(`/api/new_game/${choice}/${login_id}`)
        .then(response => {
            console.log(response.json)
        })
        .then(data => {
            result_placeholder.textContent = data
        })
}

const login = () => {
    let login_text = document.getElementById("login_input");
    let login_placeholder = document.getElementById("id_placeholder");
    console.log(login_text.value)
    login_id = login_text.value;
    login_placeholder.textContent = login_id;
}
    

const main = () => {
    document.getElementById("white")
        .addEventListener("click", chose_white);
    document.getElementById("black")
        .addEventListener("click", chose_black)
    document.getElementById("find")
        .addEventListener("click", find_pair)
    document.getElementById("login")
        .addEventListener("click", login)
}

main()
