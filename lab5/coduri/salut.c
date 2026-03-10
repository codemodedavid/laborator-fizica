#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void) {
    double a = 0.00002;
    int state = 0;
    int i, n;

    while(1) {
        printf("Tabel (1) sau (2)?\n");
        scanf("%d", &state);

        switch(state) {
            case 1: {
                double alpha_d = 0;
                double alpha_s = 0;
                double alpha_d_minute = 0;
                double alpha_s_minute = 0;
                n = 0;
                printf("DATE:\n");
                printf("ordin:\n");
                scanf("%d", &n);

                printf("alpha dreapta grade:\n");
                scanf("%lf", &alpha_d);

                printf("alpha dreapta minute:\n");
                scanf("%lf", &alpha_d_minute);

                alpha_d = alpha_d + (alpha_d_minute / 60);
                alpha_d = alpha_d * M_PI / 180;

                printf("alpha stanga grade:\n");
                scanf("%lf", &alpha_s);

                printf("alpha stanga minute:\n");
                scanf("%lf", &alpha_s_minute);

                alpha_s = alpha_s + (alpha_s_minute / 60);
                alpha_s = alpha_s * M_PI / 180;

                printf("REZULTATE:\n");
                double alpha = 0.5 * fabs(alpha_d - alpha_s);
                alpha = (alpha * 180 / M_PI);

                printf("alpha grade: %d, minute %.2f\n", (int) alpha, (alpha - (int) alpha) * 60);
                alpha = (alpha / 180 * M_PI);
                double sinus = sin(alpha);
                printf("sin: %.4f\n", sinus);
                double lambda = a/n * sinus;
                printf("lambda: %.6f\n", pow(10, 9) * lambda);
                printf("\n###############################################\n\n");
                break;
            }
            case 2: {
                double alpha_2d[10] = {}; 
                double alpha_2d_min[10] = {};       
                double alpha_2s[10] = {};
                double alpha_2s_min[10] = {};
                n = 10;
                int c = 0;
                int d = 1;

                printf("Introduceti masuratorile:\n\n");
                
                for (i=0; i<n; i++) {
                    printf("ALPHA %d:\n", i+1);
                    printf("DREAPTA %d   grade:", i+1);
                    scanf("%lf", &alpha_2d[i]);

                    printf("DREAPTA %d  minute:", i+1);
                    scanf("%lf", &alpha_2d_min[i]);

                    printf("STANGA  %d    grade:", i+1);
                    scanf("%lf", &alpha_2s[i]);

                    printf("STANGA  %d   minute:", i+1);
                    scanf("%lf", &alpha_2s_min[i]);
                    printf("corect? (0 sau 1)");
                    scanf("%d", &c);
                    
                    if (c == 0 || c != 1) {
                        i--;
                    }
                    printf("\n");

                }
                

                double alpha_2[10] = {};
                double sum = 0;

                printf("alpha_n:");

                for (i=0; i<n;i++) {
                    alpha_2d[i] = alpha_2d[i] + alpha_2d_min[i] / 60;
                    alpha_2s[i] = alpha_2s[i] + alpha_2s_min[i] / 60;

                    alpha_2[i] = 0.5 * fabs(alpha_2d[i] - alpha_2s[i]);
                    printf("%dg%dm |", (int) alpha_2[i], (int) ((alpha_2[i] - (int) alpha_2[i]) * 60));
                    sum += alpha_2[i];
                    if (i == 9) {
                        printf("%dg%dm\n", (int) alpha_2[i], (int) ((alpha_2[i] - (int) alpha_2[i]) * 60));
                    }
                }

                double alpha_avg = sum / 10;

                printf("Medie alpha: %d grade, %d minute\n", (int) alpha_avg, (int) ((alpha_avg - (int) alpha_avg) * 60));

                sum = 0;
                for (i=0; i<n;i++) {
                    sum += pow(alpha_2[i] - alpha_avg, 2);
                }
                
                double err_alpha = sqrt(sum / (n * (n-1)));
                
                printf("Eroarea alpha: %d grade, %f minute\n", (int) err_alpha, ((err_alpha - (int) err_alpha) * 60));

                alpha_avg = alpha_avg * M_PI / 180;

                double lambda_avg = a * sin(alpha_avg)/2;
                
                printf("Medie lambda: %f (nm)\n", pow(10, 9) * lambda_avg);

                double err_lambda = fabs(a/n * cos(alpha_avg)) * err_alpha;

                printf("Eroarea lambda: %f (nm)\n", pow(10, 9) * err_lambda);

                printf("lambda apartine (%f,%f)\n", pow(10, 9) * (lambda_avg - err_lambda), pow(10, 9) * (lambda_avg + err_lambda));

                printf("\n###############################################\n\n");
                break;
            }
        }
    }
}
        


