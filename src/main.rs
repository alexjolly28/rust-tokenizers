use transformers;
use transformers::preprocessing::vocab::base_vocab::Vocab;
use std::process;
use transformers::preprocessing::adapters::Example;
use transformers::preprocessing::tokenizer::bert_tokenizer::BertTokenizer;
use std::time::Instant;
use transformers::preprocessing::tokenizer::base_tokenizer::{BaseTokenizer, Tokenizer};
use transformers::BertVocab;
use std::rc::Rc;

fn main() {
    let vocab_path = "E:/Coding/rust-transformers/resources/vocab/bert-base-uncased-vocab.txt";
    let bert_vocab = Rc::new(transformers::BertVocab::from_file(vocab_path));

    let _data = match transformers::preprocessing::adapters::read_sst2(
        "E:/Coding/rust-transformers/resources/data/SST-2/train.tsv",
        b'\t') {
        Ok(examples) => {
            examples
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

//    println!("{:?}", _data);

//    let _test_sentence = Example::new_from_string("[MASK]it \'s a charming [SEP] [SEP] and often [MASK] affecting journey. [MASK]");
    let _test_sentence = Example::new_from_string("[MASK]Reprise au tout début des années [SEP]1960[SEP] par le commissariat à l'énergie atomique (CEA), cette structure reste, au xxie siècle, l'un des principaux employeurs de main d'œuvre de la commune.");
//    let _test_sentence = Example::new_from_string("[CLS]吉村洋文辭職參選日本大阪府知事後，與其同屬大阪維新會的前大阪府知事松井一郎在哪一次選舉成功接替吉村洋文的位置？");
//    println!("{:?}", _test_sentence);

    let basic_tokenizer: BaseTokenizer<BertVocab> = BaseTokenizer::from_existing_vocab(bert_vocab.clone());
    println!("{:?}", basic_tokenizer.tokenize(&_test_sentence.sentence_1));

    let bert_tokenizer: BertTokenizer<BertVocab> = BertTokenizer::from_existing_vocab(bert_vocab.clone());
    println!("{:?}", bert_tokenizer.tokenize(&_test_sentence.sentence_1));

    let _before = Instant::now();
//    for example in _data {
//        bert_tokenizer.tokenize(&example.sentence_1);
//    }
//    println!("Elapsed time: {:.2?}", _before.elapsed());

//    let test_word = String::from("unaffable");
//    let tokenized_output = tokenize_wordpiece(test_word, &bert_vocab, 100);
//    println!("{:?}", tokenized_output);
}
