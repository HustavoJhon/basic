using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace Calculadora
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    
    public enum OperacionSeleccionada
    {
        Suma,
        Resta,
        Multiplicacion,
        Division
    }

    public class Calculador
    {
        public static double Suma(double n1 , double n2)
        {
            return n1 + n2;
        }
        public static double Resta(double n1 , double n2)
        {
            return n1 - n2;
        }
        public static double Multiplicacion(double n1 , double n2)
        {
            return n1 * n2;
        }
        public static double Division(double n1 , double n2)
        {
            if(n2 == 0)
            {
                MessageBox.Show(
                    "La division por 0 no esta definida",
                    "Operacion incorrecta",
                    MessageBoxButton.OK,
                    MessageBoxImage.Error
                    );
                return 0;
            }
            return n1 / n2;
        }
    }

    public partial class MainWindow : Window
    {
        double numeroAnterior, resultado;

        OperacionSeleccionada operacionSeleccionada;


        public MainWindow()
        {
            InitializeComponent();
            acBoton.Click += AcBoton_Click;
            negativoBoton.Click += NegativoBoton_Click;
            porcentajeBoton.Click += PorcentajeBoton_Click;
            igualBoton.Click += IgualBoton_Click;
            puntoBoton.Click += PuntoBoton_Click;
        }

        private void PuntoBoton_Click(object sender, RoutedEventArgs e)
        {
            if(double.TryParse(resultadoLabel.Content.ToString().Contains(",")))
            {

            }
            else
            {
                resultadoLabel.Content = $"{resultadoLabel.Content},";
            }
            resultadoLabel.Content = double.TryParse(resultadoLabel.Content.ToString().Contains(",") ? resultadoLabel.Content: $"{resultadoLabel.Content},";
        }

        private void IgualBoton_Click(object sender, RoutedEventArgs e)
        {
            double numeroNuevo;
            if (double.TryParse(resultadoLabel.Content.ToString(), out numeroNuevo))
            {
                switch (operacionSeleccionada)
                {
                    case OperacionSeleccionada.Suma:
                        resultado = Calculador.Suma(numeroAnterior, numeroNuevo);
                        break;
                    case OperacionSeleccionada.Resta:
                        resultado = Calculador.Resta(numeroAnterior, numeroNuevo);
                        break;
                    case OperacionSeleccionada.Multiplicacion:
                        resultado = Calculador.Multiplicacion(numeroAnterior, numeroNuevo);
                        break;
                    case OperacionSeleccionada.Division:
                        resultado = Calculador.Division(numeroAnterior, numeroNuevo);
                        break;
                }
            }

            resultadoLabel = $"{resultado}";
        }

        private void OperacionBoton_Click(object sender, RoutedEventArgs e)
        {

            //eso vuelve el valor mostrado 0 y guardo la variable
            if (double.TryParse(resultadoLabel.Content.ToString(), out numeroAnterior))
            {
                resultadoLabel.Content = "0";
            }

            //Con esto selecciono la operacion que realizare cuando le de al igual
            operacionSeleccionada = sender == multiplicacionBoton ? OperacionSeleccionada.Multiplicacion : operacionSeleccionada;
            operacionSeleccionada = sender == divisionBoton ? OperacionSeleccionada.Division : operacionSeleccionada;
            operacionSeleccionada = sender == menosBoton ? OperacionSeleccionada.Resta : operacionSeleccionada;
            operacionSeleccionada = sender == masBoton ? OperacionSeleccionada.Suma : operacionSeleccionada;

        }

        private void NumeroBoton_Click(object sender, RoutedEventArgs e)
        {
            int valorSeleccionado = int.Parse((sender as Button).Content.ToString());

            resultadoLabel.Content = resultadoLabel.Content.ToString() == "0" ? $"{valorSeleccionado}" : $"{resultadoLabel.Content}{valorSeleccionado}";
            //if(resultadoLabel.Content.ToString() == "0")
            //{
            //    resultadoLabel.Content = $"{valorSeleccionado}";
            //}
            //else
            //{
            //    resultadoLabel.Content = $"{resultadoLabel.Content}{valorSeleccionado}";
            //}
        }

        private void PorcentajeBoton_Click(object sender, RoutedEventArgs e)
        {
            if (double.TryParse(resultadoLabel.Content.ToString(), out numeroAnterior))
            {
                numeroAnterior = numeroAnterior / 100;
                resultadoLabel.Content = numeroAnterior.ToString();
            }
        }

        private void NegativoBoton_Click(object sender, RoutedEventArgs e)
        {
            //volver negativo el resultado que se muestra en la pantalla
            if (double.TryParse(resultadoLabel.Content.ToString(), out numeroAnterior))
            {
                numeroAnterior = numeroAnterior * -1;
                resultadoLabel.Content = numeroAnterior.ToString();
            }
        }

        private void AcBoton_Click(object sender, RoutedEventArgs e)
        {
            // Rorrar el resultado que se muestra en la pantalla
            resultadoLabel.Content = "0";
        }
    }
}
