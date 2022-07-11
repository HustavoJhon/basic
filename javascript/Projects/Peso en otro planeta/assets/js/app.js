function here(id) {
    alert("Como te llamas");
    let name = prompt(">>");
    alert ("Hola " + name);
}

// peso de el usuario
var usuario = parseInt(prompt("Cuanto Pesas?"));

// menu
var menu = parseInt(prompt("Escoje tu planeta: \n1:Mercurio\n2:Venus\n3:Tierra\n4:Marte\n5:Jupiter\n6:Saturno\n7:Urano\n8:Nepturno"))

var planet = planeta_choice;
var g_mercurio = 3.7;
var g_venus = 8.83;
var g_tierra = 9.8;
var g_marte = 3.71;
var g_jupiter = 24.8; 
var g_saturno = 10.44;
var g_urano = 8.83;
var g_nepturno = 11.15;

if (planeta == 1){
    peso_final = peso * g_mercurio / g_tierra;
} else if(planeta == 2) {
    peso_final = peso * g_venus / g_tierra;
} else{
    alert("bye")
}


document.write("Tu en " + name_planeta + " pesa <strong>" + peso + "kilos </strong>");

