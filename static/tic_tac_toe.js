let choice = null
let login_id = null

const login = () => {
    let login_text = document.getElementById("login_input");
    let login_placeholder = document.getElementById("id_placeholder");
    console.log(login_text.value)
    login_id = login_text.value;
    login_placeholder.textContent = login_id;
}

const chose_xs = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "Xs";
    placeholder.textContent = choice;
    console.log(choice);
}

const chose_os = () => {
    let placeholder = document.getElementById("choice_placeholder");
    choice = "Os";
    placeholder.textContent = choice;
    console.log(choice);
}

const find_pair = () => {
    let result_placeholder = document.getElementById("result");
    const socket = new WebSocket(`ws://localhost:5000/api/tic_tac_toe/new_session/${login_id}`)
    socket.addEventListener('open', _event => {
        console.log(_event);
        socket.send(`/find?${choice}`);
    })
    socket.addEventListener('message', (msg) => {
        console.log("Message from server: ", msg.data);
        result_placeholder.textContent = msg.data;
    })
}


const main = () => {
    document.getElementById("Xs")
        .addEventListener("click", chose_xs);
    document.getElementById("Os")
        .addEventListener("click", chose_os)
    document.getElementById("find")
        .addEventListener("click", find_pair)
    document.getElementById("login")
        .addEventListener("click", login)
}

main()
