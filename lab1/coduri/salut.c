#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void) {
    double a = 0.02;
    int state = 0;
    int i, n;

    while(1) {
        printf("Tabel (1) sau (2)?\n");
        scanf("%d", &state);

        switch(state) {
            case 1: {
                double alpha_d = 0;
                double alpha_s = 0;
                n = 0;
                printf("DATE:\n");
                printf("ordin:\n");
                scanf("%d", &n);
                printf("alpha dreapta:\n");
                scanf("%lf", &alpha_d);
                alpha_d = alpha_d * M_PI / 180;
                printf("alpha stanga:\n");
                scanf("%lf", &alpha_s);
                alpha_s = alpha_s * M_PI / 180;

                printf("REZULTATE:\n");
                double alpha = 0.5 * fabs(alpha_d - alpha_s);
                printf("alpha: %.4f\n", alpha);
                double sinus = sin(alpha);
                printf("sin: %.4f\n", sinus);
                double lambda = a/n * sinus;
                printf("lambda: %.4f\n", lambda);
                printf("\n###############################################\n\n");
                break;
            }
            case 2: {
                n = 10;
                double alpha_2d[10] = {10, 32, 43, 100, 23, 22, 10, 11, 94, 169};
                double alpha_2s[10] = {33, 76, 45 ,5 ,100, 122, 31, 87, 92, 199};
                double alpha_2[10] = {};
                double sum = 0;

                printf("alpha_n:");
                for (i=0; i<n;i++) {
                    alpha_2[i] = 0.5 * fabs(alpha_2d[i] - alpha_2s[i]);
                    printf("%f,", alpha_2[i]);
                    sum += alpha_2[i];
                    if (i == 9) {
                        printf("%f\n", alpha_2[i]);
                    }
                }

                double alpha_avg = sum / 10;

                printf("Medie alpha: %f\n", alpha_avg);

                sum = 0;
                for (i=0; i<n;i++) {
                    sum += pow(alpha_2[i] - alpha_avg, 2);
                }
                
                double err_alpha = sqrt(sum / (n * (n-1)));
                
                printf("Eroarea alpha: %f\n", err_alpha);

                alpha_avg = alpha_avg * M_PI / 180;

                double lambda_avg = a/n * sin(alpha_avg);
                
                printf("Medie lambda: %f\n", lambda_avg);

                double err_lambda = fabs(a/n * cos(alpha_avg)) * err_alpha;

                printf("Eroarea lambda: %f\n", err_lambda);

                printf("lambda apartine (%f,%f)\n", (lambda_avg - err_lambda), (lambda_avg + err_lambda));

                printf("\n###############################################\n\n");
                break;
            }
        }
        
    }
}

