let choice = null
let login_id = null

const login = () => {
    let login_text = document.getElementById("login_input");
    let login_placeholder = document.getElementById("id_placeholder");
    console.log(login_text.value)
    login_id = login_text.value;
    login_placeholder.textContent = login_id;
}

const chose_black = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "white";
    placeholder.textContent = choice;
    console.log(choice);
}

const chose_white = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "black";
    placeholder.textContent = choice;
    console.log(choice);
}

const find_pair = () => {
    let result_placeholder = document.getElementById("result");
    const socket = new WebSocket(`ws://localhost:8000/api/chess/new_game/${login_id}/${choice}`)
    socket.addEventListener('open', _event => {
        socket.send("/find");
    })
    socket.addEventListener('message', (msg) => {
        console.log("Message from server: ", msg.data);
        result_placeholder.textContent = msg.data;
    })
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
