use std::io;
use rand::Rng;

fn main() {
    let pontuacao_minima = 17;
    let pontuacao_maxima = 21;
    let vlr_aposta_minima = 1;
    let qtd_naipes = 4;
    let qtd_cartas = 13;
    let num_decks = 4;
    let qtd_minima_cartas_no_deck = 10;
    let mut deck = criar_baralho(num_decks, qtd_naipes, qtd_cartas);
    let mut resultado: String = String::from("");

    println!("Digite o valor inicial para apostar:");
    let dinheiro_jogador_inicial = le_inteiro();
    let mut dinheiro_jogador = dinheiro_jogador_inicial;

    loop {
        if deck.len() < qtd_minima_cartas_no_deck {
            println!("Acabaram as cartas, jogo encerrado!");
            break;
        }

        if dinheiro_jogador == 0 {
            println!("Você ficou sem dinheiro, jogo encerrado!");
            break;
        }

        println!("Você tem {} de dinheiro. Deseja continuar jogando? (s/n)", dinheiro_jogador);
        let continuar = le_string();
        if continuar.trim().to_lowercase() != "s" {
            println!("Jogo encerrado por opção do jogador!");
            break;
        }

        println!("Digite o valor da aposta (min 1):");
        let aposta = le_inteiro();
        if aposta < vlr_aposta_minima || aposta > dinheiro_jogador {
            println!("Aposta inválida. Tente novamente.");
            continue;
        }

        let mut mao_jogador = Vec::new();
        let mut mao_mesa = Vec::new();

        comprar_carta(&mut deck, &mut mao_jogador);
        comprar_carta(&mut deck, &mut mao_jogador);
        comprar_carta(&mut deck, &mut mao_mesa);
        comprar_carta(&mut deck, &mut mao_mesa);

        // Turno do jogador
        loop {
            let p_jogador = pontuacao_jogador(mao_jogador.clone());
            println!("Suas cartas: {:?} (pontuação jogador: {})", mao_jogador, p_jogador);
            if p_jogador > pontuacao_maxima {
                println!("Você estourou! Pontos: {}", p_jogador);
                dinheiro_jogador -= aposta;
                break;
            }

            println!("Deseja comprar mais cartas? (s/n)");
            let opcao = le_string();
            if opcao.trim().to_lowercase() != "s" {
                break;
            }

            comprar_carta(&mut deck, &mut mao_jogador);
        }

        let p_jogador = pontuacao_jogador(mao_jogador.clone());
        if p_jogador > pontuacao_maxima {
            resultado = format!("Jogador estourou! Pontos do Jogador: {}", p_jogador);
            break;
        }

        if p_jogador == pontuacao_maxima {
            resultado = format!("Jogador venceu! Pontos do Jogador: {}", p_jogador);
            break;
        }

        let p_mesa_temp = pontuacao_mesa(mao_mesa.clone());
        if p_jogador == p_mesa_temp {
            resultado = "Empate!".to_string();
            break;
        }

        // Turno da mesa
        loop {
            let p_mesa = pontuacao_mesa(mao_mesa.clone());
            if p_mesa > pontuacao_maxima {
                println!("Mesa estourou! Pontos da mesa: {}", p_mesa);
                dinheiro_jogador += aposta;
                break;
            }

            if p_mesa >= pontuacao_minima {
                println!("A mesa parou de comprar. Pontos da mesa: {}", p_mesa);
                if p_mesa > p_jogador {
                    println!("A mesa ganhou! {} vs {}", p_mesa, p_jogador);
                    dinheiro_jogador -= aposta;
                } else if p_mesa < p_jogador {
                    println!("Você ganhou! {} vs {}", p_jogador, p_mesa);
                    dinheiro_jogador += aposta;
                } else {
                    println!("Empate! Ninguém ganha ou perde.");
                }
                break;
            } else {
                comprar_carta(&mut deck, &mut mao_mesa);
            }
        }
    }

    println!("Resultado do jogo {}.", resultado);
    println!("Você terminou com {} de dinheiro.", dinheiro_jogador);
    println!("Obrigado por jogar!");
}

fn criar_baralho(num_decks: i32, qtd_naipes: i32, qtd_cartas: i32) -> Vec<i32> {
    let mut deck = Vec::new();
    for _ in 0..num_decks {
        for _ in 0..qtd_naipes {
            for carta in 1..=qtd_cartas {
                deck.push(carta);
            }
        }
    }
    deck
}

fn comprar_carta(deck: &mut Vec<i32>, mao: &mut Vec<i32>) {
    if deck.is_empty() {
        return;
    }
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..deck.len());
    let carta = deck.remove(idx);
    mao.push(carta);
}

fn pontuacao_jogador(mao: Vec<i32>) -> i32 {
    let mut total = 0;
    for carta in mao {
        total += valor_carta(carta);
    }
    total
}

fn pontuacao_mesa(mao: Vec<i32>) -> i32 {
    let mut total = 0;
    for carta in mao {
        total += valor_carta(carta);
    }
    total
}

fn valor_carta(carta: i32) -> i32 {
    if carta >= 11 && carta <= 13 {
        10 
    } else if carta == 1{
        11
    } else {
        carta 
    }
}

fn le_inteiro() -> i32 {
    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada); // Ignora o Result
    let numero: i32 = entrada.trim().parse().unwrap();
    numero
}

fn le_string() -> String {
    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada); // Ignora o Result
    entrada
}