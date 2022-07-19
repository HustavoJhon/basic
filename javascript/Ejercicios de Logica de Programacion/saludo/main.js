// const name = document.getElementById('name');

function saludar() {
    const inputName = document.getElementById('username');
    const nameuser = inputName.value;
    alert(`Hola ${nameuser}`);
}

function saludarAll() {
    const inputName = document.getElementById('username');
    const nameuser = inputName.value;

    const inputFriend = document.getElementById('friendname');
    const nameFriend = inputFriend.value;

    alert(`${nameuser} te saluda ${nameFriend}`);
}
