/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */
public class cliente {
    private String Nombre;
    private int DNI;
    private int Edad;
    
    public cliente() {
        
    }
    
    public cliente(String nombre, int dni,  int edad) {
        this.Nombre = nombre;
        this.DNI = dni;
        this.Edad = edad;
    }

    public void setNombre(String Nombre) {
        this.Nombre = Nombre;
    }

    public void setDNI(int DNI) {
        this.DNI = DNI;
    }

    public void setEdad(int Edad) {
        this.Edad = Edad;
    }

    public String getNombre() {
        return Nombre;
    }

    public int getDNI() {
        return DNI;
    }

    public int getEdad() {
        return Edad;
    }
    
    
}
