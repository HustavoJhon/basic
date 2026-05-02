<?PHP
	class usuarios{
		// Atributos
		public $id;
		public $usuario;
		public $contrasena;
		// Recuperacion de atributos
		function get_id(){
			return $this->id;
		}
		function get_usuario(){
			return $this->usuario;
		}
		function get_contrasena(){
			return $this->contrasena;
		}
		// Asignacion de atributos	
		function set_id($id){
			$this->id = $id;
		}
		function set_usuario($usuario){
			$this->usuario = $usuario;
		}
		function set_contrasena($contrasena){
			$this->contrasena = $contrasena;
		}

	}
?>
