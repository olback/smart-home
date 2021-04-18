import { Component, OnInit } from '@angular/core';
import { ChartDataSets, ChartOptions, ChartHoverOptions } from 'chart.js';
import { Color, Label } from 'ng2-charts';

interface CompleteChartSetup {
  cardTitle: string,
  charts: {
    lineChartData: ChartDataSets[];
    lineChartLabels: Label[];
    lineChartOptions: ChartOptions;
    lineChartColors: Color[];
    lineChartLegend: boolean;
    lineChartPlugins: any[];
    lineChartType: string;
  }[]
}

@Component({
  selector: 'app-temperature',
  templateUrl: './temperature.component.html',
  styleUrls: ['./temperature.component.scss']
})
export class TemperatureComponent implements OnInit {

  cards: CompleteChartSetup[] = [
    {
      cardTitle: 'Temperature and humidity inside',
      charts: [
        {
          lineChartData: [
            { data: [22, 19, 25, 24, 20, 23], label: 'Temperature' },
          ],
          lineChartLabels: ['January', 'February', 'March', 'April', 'May', 'June'],
          lineChartOptions: {
            responsive: true,
            aspectRatio: 10
          },
          lineChartColors: [
            {
              borderColor: 'black',
              backgroundColor: 'rgba(0,255,0,0.28)',
              borderWidth: 1
            },
          ],
          lineChartLegend: true,
          lineChartPlugins: [],
          lineChartType: 'line'
        },
        {
          lineChartData: [
            { data: [22, 19, 25, 24, 20, 23], label: 'Humidity' },
          ],
          lineChartLabels: ['January', 'February', 'March', 'April', 'May', 'June'],
          lineChartOptions: {
            responsive: true,
            aspectRatio: 10
          },
          lineChartColors: [
            {
              borderColor: 'black',
              backgroundColor: 'rgba(0,0,255,0.28)',
              borderWidth: 1
            },
          ],
          lineChartLegend: true,
          lineChartPlugins: [],
          lineChartType: 'line'
        }
      ]
    },
    {
      cardTitle: 'Temperature and humidity outside',
      charts: [
        {
          lineChartData: [
            { data: [1, 3, 7, 5, 10, 2], label: 'Temperature' },
          ],
          lineChartLabels: ['January', 'February', 'March', 'April', 'May', 'June'],
          lineChartOptions: {
            responsive: true,
            aspectRatio: 10
          },
          lineChartColors: [
            {
              borderColor: 'black',
              backgroundColor: 'rgba(0,255,0,0.28)',
              borderWidth: 1
            },
          ],
          lineChartLegend: true,
          lineChartPlugins: [],
          lineChartType: 'line'
        },
        {
          lineChartData: [
            { data: [22, 19, 25, 24, 20, 23], label: 'Humidity' },
          ],
          lineChartLabels: ['January', 'February', 'March', 'April', 'May', 'June'],
          lineChartOptions: {
            responsive: true,
            aspectRatio: 10
          },
          lineChartColors: [
            {
              borderColor: 'black',
              backgroundColor: 'rgba(0,0,255,0.28)',
              borderWidth: 1
            },
          ],
          lineChartLegend: true,
          lineChartPlugins: [],
          lineChartType: 'line'
        }
      ]
    }
  ];

  constructor() { }

  ngOnInit(): void {
  }

}
