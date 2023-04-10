void main() {
  // Mascotas

  int mascotas = 2;

  if (mascotas == 0) {
    print("No tienes mascotas");
  } else {
    if (mascotas == 1) {
      print("Solo tines una mascota");
    } else {
      print("Tienes muchas mascotas");
    }
  }

  // SWITCH

  switch (mascotas) {
    case 0:
      print("No tines mascotas");
      break;
    case 1:
      print("Solo tiene una mascota");
      break;
    default:
      print("Tienes muchas mascotas");
  }
}
