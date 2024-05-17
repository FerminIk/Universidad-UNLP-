/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */
public class Autor {
    private String Nombre;
    private String Biografia;
    private String Origen;

    public Autor(String Nombre, String Biografia, String Origen) {
        this.Nombre = Nombre;
        this.Biografia = Biografia;
        this.Origen = Origen;
    }
    
    public Autor(){
        
    }

    public String getNombre() {
        return Nombre;
    }

    public String getBiografia() {
        return Biografia;
    }

    public String getOrigen() {
        return Origen;
    }

    public void setNombre(String Nombre) {
        this.Nombre = Nombre;
    }

    public void setBiografia(String Biografia) {
        this.Biografia = Biografia;
    }

    public void setOrigen(String Origen) {
        this.Origen = Origen;
    }

    @Override
    public String toString() {
        return "Autor{" + "Nombre=" + Nombre + ", Biografia=" + Biografia + ", Origen=" + Origen + '}';
    }
    
    
}
