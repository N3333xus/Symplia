# AFDs - Autômatos Finitos Determinísticos (Semana 5)

📋 Objetivos da Semana:

Converter expressões regulares em autômatos finitos determinísticos para implementação eficiente.

🎯 Atividades do Projeto Integrador:

Você implementará algoritmos para converter as expressões regulares da semana anterior em autômatos finitos determinísticos funcionais. Este processo conectará diretamente teoria e prática, mostrando como conceitos matemáticos se materializam em código funcional.

Comece criando um diagrama para cada AFD a partir das expressões regulares. Depois implemente AFDs usando técnicas de construção sistemática. Para expressões mais complexas, você pode primeiro criar representações intermediárias mais simples de visualizar e depois otimizar para implementações eficientes. Teste cada autômato individualmente antes de integrá-los em um analisador léxico unificado.

Você também implementará o algoritmo de minimização de autômatos, observando como ele reduz o tamanho dos AFDs sem afetar a funcionalidade. Esta otimização será importante para a eficiência do compilador final. Experimente com diferentes estratégias de implementação: tabelas de transição são eficientes em velocidade mas podem consumir muita memória, enquanto implementações baseadas em código podem ser mais compactas.

💡 Reflexões Importantes:

Esta é uma semana particularmente importante porque você verá como algoritmos abstratos se transformam em código concreto. Preste atenção a como pequenos detalhes de implementação podem afetar drasticamente a performance. Um autômato mal construído pode tornar a análise léxica lenta, impactando toda a experiência de uso do compilador.

Como decidir qual estratégia de implementação usar? Considere o trade-off entre velocidade de execução e uso de memória. Para um compilador educacional, talvez a clareza de código seja mais importante que a otimização extrema. Para um compilador de produção, a eficiência pode ser essencial.

📊 Entrega da Semana:

    Diagrama de cada AFD;
    Implementação funcional dos AFDs
    Testes unitários para cada autômato individual

👉 Crie um arquivo de diagrama (markdown com mermaid) para AFD. 👉 A entrega é o link para o commit final. 👉 Fique atento aos horários de abertura para entrega e de fechamento da tarefa!!!

😉 Bom trabalho 😉