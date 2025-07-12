<p align="center">
  <img alt="" src="https://raw.githubusercontent.com/skippyr/river_dreams/refs/heads/master/assets/ornament.png" width="1020" />
</p>
<h1 align="center">≥v≥v&ensp;River Dreams&ensp;≥v≥v</h1>
<p align="center">
  <img alt="" src="https://img.shields.io/github/license/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20licen%C3%A7a&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="" src="https://img.shields.io/github/v/tag/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20tag&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="" src="https://img.shields.io/github/commit-activity/t/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20commits&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="" src="https://img.shields.io/github/stars/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20estrelas&labelColor=%2324130e&color=%23b8150d" />
</p>
<p align="center">
  <img alt="" src="https://img.shields.io/badge/macOS-000000?logo=apple&logoColor=F0F0F0" />
  <img alt="" src="https://img.shields.io/badge/Linux-FCC624?logo=linux&logoColor=black" />
</p>
<p align="center">
  <span><a href="https://github.com/skippyr/river_dreams/blob/master/README.md">🇺🇸 English-US</a></span>
  &ensp;
  <span><a href="https://github.com/skippyr/river_dreams/blob/master/README_pt-BR.md">🇧🇷 Portuguese-BR</a></span>
</p>
<p align="center"><sup>Desde Fevereiro de 2023</sup></p>

## ❡ Sobre
Um tema de aparência tribal para o shell ZSH confeccionado para ajudar você a dar vida a seus projetos de software mais ambiciosos. Ele está disponível para macOS e Linux.

<p align="center">
  <img alt="" src="https://raw.githubusercontent.com/skippyr/river_dreams/refs/heads/master/assets/preview.png" width="1020" />
</p>
<p align="center"><strong>Legenda:</strong> uma prévia do River Dreams executando <a href="https://github.com/eza-community/eza">eza</a> no terminal <a href="https://github.com/kovidgoyal/kitty">Kitty</a> do macOS. A paleta de cores utilizada é <a href="https://github.com/skippyr/flamerial">Flamerial</a> e a fonte é <a href="https://www.monolisa.dev">Monolisa</a> (com substituição alternativa para <a href="https://github.com/ryanoasis/nerd-fonts">Symbols Nerd Font</a>). Imagem de fundo é uma arte de IA proveniente de um porte de wallpaper da paleta <a href="https://github.com/skippyr/flamerial">Flamerial</a>.</p>

## ❡ Funcionalidades
O prompt esquerdo mostra:

<p>
  <details>
    <summary>Clique para expandir (...)</summary>
    <ul>
      <li>Seu endereço IP na rede local.</li>
      <li>Seu uso do disco de armazenamento e seu status.</li>
      <li>Sua carga de bateria restante e seu status, se disponível.</li>
      <li>Um calendário que mostra o dia da semana, mês e dia do mês.</li>
      <li>Um relógio de 24 horas que mostra as horas e minutos.</li>
      <li>Um indicador quando você é o usuário root.</li>
      <li>O código de retorno do último comando.</li>
      <li>O nome do ambiente virtual Python ativo, se um foi selecionado.</li>
      <li>O caminho do diretório atual, abreviado quando dentro de repositórios Git.</li>
      <li>O nome da branch Git ativa e um indicador quando estiver dirty, quando dentro de repositórios Git.</li>
      <li>Um indicador quando você não tem posse do diretório atual.</li>
    </ul>
  </details>
</p>

O prompt direito mostra:

<p>
  <details>
    <summary>Clique para expandir (...)</summary>
    <ul>
      <li>
        O número total de cada tipo de entrada presente no diretório atual:
        <ul>
          <li>Arquivos.</li>
          <li>Diretórios.</li>
          <li>Sockets.</li>
          <li>Fifos.</li>
          <li>Dispositivos de bloco</li>
          <li>Dispositivos de caractere.</li>
          <li>Links simbólicos <em>(eles não são seguidos)</em>.</li>
          <li>Entradas ocultas.</li>
          <li>Entradas temporárias.</li>
        </ul>
      </li>
      <li>O número total de tarefas rodando em segundo plano.</li>
    </ul>
  </details>
</p>

## ❡ Instalação
### Dependências
As seguintes dependências devem ser instaladas inicialmente:
- **ZSH**: este é o shell no qual o tema roda. Se você está usando o macOS, ele já é a sua opção padrão. Se estiver no Linux, algumas distros utilizam-o como padrão, caso não seja o seu caso, você pode trocar para ele manualmente.
- [**Toolchain do Rust**](https://www.rust-lang.org): ela será utilizada para compilar os arquivos de código-fonte.
- **Uma fonte modificada pelo [projeto Nerd Fonts](https://www.nerdfonts.com/font-downloads):** ela prove todos os símbolos estilizados utilizados pelo software. Como alternativa, você pode usar uma fonte comum mas com substituição alternativa para a fonte que contém apenas tais símbols se o seu terminal suportar. Evite instalar multiplas fontes modificadas para evitar conflitos de renderização.
- **Um terminal com ótimo suporte ao Unicode:** ele será utilizado para executar o shell. É fortemente recomendado usar o [**Kitty**](https://github.com/kovidgoyal/kitty) devido as suas incríveis gama de recursos e renderização de símbolos.

Use o administrador de pacotes do seu SO ou use o [HomeBrew](https://brew.sh) para instalar estes pacotes.

### Procedimentos
- Aplique a Nerd Font instalada em seu terminal.
- Instale o tema usando `cargo`. Caso ocorra algum erro, verifique a saída do `cargo` para identificar o motivo.

```zsh
cargo install river_dreams;
```

- Inicialize o tema em seu `~/.zshrc`:

```zsh
echo 'eval $(river_dreams init);' >> ~/.zshrc;
```

- Reabra o seu shell.

## ❡ Ajuda
Caso precise de ajuda relacionada a este projeto, abra uma nova issue em sua [página de issues](https://github.com/skippyr/river_dreams/issues) ou envie um [e-mail](mailto:skippyr.developer@icloud.com) relatando o que está acontecendo.

## ❡ Contribuições
Este projeto está disposto a analizar e potencialmente aceitar contribuições na forma de identificação de bugs e sugestões. Se estiver interessado, envie sua contribução para sua [página de pull requests](https://github.com/skippyr/river_dreams/pulls) ou via [e-mail](mailto:skippyr.developer@icloud.com).

## ❡ Agradecimentos Especiais
Um grande abraço ao [unixorn](https://github.com/unixorn) que viu potencial neste software e o considerou com um awesome plugin. Se você quiser ver outras criações incríveis feitas por outros desenvolvedores, não se limitando apenas a temas, lembre-se de visitar o [repositório Awesome ZSH Plugins](https://github.com/unixorn/awesome-zsh-plugins).

## ❡ Licença
Este é um software livre licensiado sob a licença BSD-3-Clause e que NÃO APRESENTA GARANTIA. Consulte o arquivo `LICENSE` que vem com o código-fonte para mais detalhes da licença e direitos de uso.

&ensp;
<p align="center"><sup>– 🐉❤️‍🔥 –</br><strong>≥v≥v&ensp;Aqui Vivem Os Dragões!&ensp;≥v≥</strong><br/>Feito com amor por 🍒 <a href="https://github.com/skippyr">skippyr</a></sup></p>
