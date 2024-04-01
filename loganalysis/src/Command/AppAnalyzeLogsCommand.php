<?php

namespace App\Command;

use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;

#[AsCommand(
    name: 'app:analyze-logs',
    description: 'Add a short description for your command',
)]
class AppAnalyzeLogsCommand extends Command
{
    public function __construct()
    {
        parent::__construct();
    }

    protected function configure(): void
    {
        $this
            ->setDescription('Analyse les logs et affiche des statistiques.')
            ->addArgument('filePath', InputArgument::REQUIRED, 'Le chemin du fichier de logs à analyser.');
    }

    protected function execute(InputInterface $d0,OutputInterface $j1):int{$x2=microtime(true);$o3=$d0->getArgument(base64_decode('ZmlsZVBhdGg='));$q4=fopen($o3,base64_decode('cg=='));if(!$q4){$j1->writeln(base64_decode('SW1wb3NzaWJsZSBkXCdvdXZyaXIgbGUgZmljaGllciBkZSBsb2dzLg=='));return Command::FAILURE;}$d5=0;$r6=[];$t7=[];while(($i8=fgets($q4))!==false){++$d5;preg_match(base64_decode('L14oXFMrKS8='),$i8,$h9);$ba=$h9[1]?? base64_decode('dW5rbm93bg==');if(!array_key_exists($ba,$t7)){$t7[$ba]=0;}++$t7[$ba];preg_match(base64_decode('LyJccyhcZHszfSlccy8='),$i8,$eb);$qc=$eb[1]?? base64_decode('dW5rbm93bg==');if(!array_key_exists($qc,$r6)){$r6[$qc]=0;}++$r6[$qc];}fclose($q4);$hd=array_keys($t7,max($t7))[0];$ee=$t7[$hd];$vf=($ee/$d5)*100;$j1->writeln("Nombre total de requêtes: $d5");foreach($r6 as $t10=>$m11){$j1->writeln("Statut HTTP $t10 : $m11");}$u12=count($t7);$j1->writeln("Nombre total d'adresses IP uniques : $u12");$j1->writeln("Adresse IP la plus fréquente : $hd ({$ee} requêtes, ".number_format($vf,2).base64_decode('JSBkdSB0b3RhbCk='));$k13=microtime(true);$v14=$k13-$x2;$j1->writeln("Temps d'exécution du script: {$v14} secondes.");return Command::SUCCESS;}
}
