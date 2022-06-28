var usuario = prompt("Cuanto Pesas?");
var peso = parseInt(usuario);

var planeta_choice = prompt("Escoge tu planeta \n1=►Marte\n2=►venus\n3=►jupiter");

var planet = planet_choice;
var g_mercurio = 3.7;
var g_venus = 8.83;
var g_tierra = 9.8;
var g_marte = 3.71;
var g_jupiter = 24.8; 
var g_saturno = 10.44;
var g_urano = 8.83;
var g_nepturno = 11.15;

switch (planeta) {
    case 'Marte': case 'marte':
        var g_plante = g_marte
    break;

    default:
        break;
}