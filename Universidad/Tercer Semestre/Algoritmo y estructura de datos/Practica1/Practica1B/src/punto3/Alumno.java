package punto3;

public class Alumno {
	private String nombre;
	private String apellido;
	private String comision;
	private String email;
	private int direccion;
	
	
	public Alumno(String nombre, String apellido, String comision, String email, int direccion) {
		super();
		this.nombre = nombre;
		this.apellido = apellido;
		this.comision = comision;
		this.email = email;
		this.direccion = direccion;
	}


	public String getNombre() {
		return nombre;
	}


	public void setNombre(String nombre) {
		this.nombre = nombre;
	}


	public String getApellido() {
		return apellido;
	}


	public void setApellido(String apellido) {
		this.apellido = apellido;
	}


	public String getComision() {
		return comision;
	}


	public void setComision(String comision) {
		this.comision = comision;
	}


	public String getEmail() {
		return email;
	}


	public void setEmail(String email) {
		this.email = email;
	}


	public int getDireccion() {
		return direccion;
	}


	public void setDireccion(int direccion) {
		this.direccion = direccion;
	}


	public String tusDatos() {
		return "Alumno [nombre=" + nombre + ", apellido=" + apellido + ", comision=" + comision + ", email=" + email
				+ ", direccion=" + direccion + "]";
	}
	
	
	
}
