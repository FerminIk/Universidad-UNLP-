/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */
public class Hotel {
    private int DF;
    private int DL;
    private Habitacion[] Habitaciones;

    public Hotel(int DF) {
        this.DF = DF;
        this.DL = 0;
        this.Habitaciones = new Habitacion[DF];
    }

    public Hotel() {
    }

    public void inicializar() {
        for (int i=0;i<DF;i++){
            Habitaciones[i] = new Habitacion();
        }
    }
    
    public void setDF(int DF) {
        this.DF = DF;
    }

    public void setDL(int DL) {
        this.DL = DL;
    }
    
    public void incrementarDL() {
        this.setDL(this.getDL() + 1);
    }

    public void setHabitaciones(Habitacion[] Habitaciones) {
        this.Habitaciones = Habitaciones;
    }
   

    public int getDF() {
        return DF;
    }

    public int getDL() {
        return DL;
    }

    public Habitacion[] getHabitaciones() {
        return Habitaciones;
    }
    
    public void IngresarCliente(cliente c,int nro){
        if  (!Habitaciones[nro].isOcupado()){
            this.Habitaciones[nro].setCliente(c);
        } else {
            System.out.println("Habitacion ocupada");
        }
    }
    
    public void aumentarPrecio(double cant){
        for (int  i=0;i<DF;i++){
            double aux;
            if (Habitaciones[i].isOcupado()){
                aux = Habitaciones[i].getCostoPorNoche() + cant;
                Habitaciones[i].setCostoPorNoche(aux);
            }
        }
    }

    public void imprimir(){
        
        for (int  i=0;i<DF;i++){
            if (Habitaciones[i].isOcupado()){
                System.out.println("Habitación " + i + " : " + Habitaciones[i].getCostoPorNoche() +" , ocupada,"+ Habitaciones[i].getCliente().getNombre() + " : "+ Habitaciones[i].getCliente().getDNI() + " : "+ Habitaciones[i].getCliente().getEdad() + " . ");
            } else {
                System.out.println("Habitación " + i + " : " + Habitaciones[i].getCostoPorNoche() +" , libre,");
            }
        }
    }
}
