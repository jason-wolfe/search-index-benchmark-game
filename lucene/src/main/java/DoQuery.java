import org.apache.lucene.analysis.standard.StandardAnalyzer;
import org.apache.lucene.index.DirectoryReader;
import org.apache.lucene.index.IndexReader;
import org.apache.lucene.queryparser.classic.ParseException;
import org.apache.lucene.queryparser.classic.QueryParser;
import org.apache.lucene.search.IndexSearcher;
import org.apache.lucene.search.Query;
import org.apache.lucene.store.FSDirectory;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Paths;

public class DoQuery {
    public static void main(String[] args) throws IOException, ParseException {
        System.err.println("starting");
        try (IndexReader reader = DirectoryReader.open(FSDirectory.open(Paths.get("/tmp/wiki_index_lucene")))) {
            IndexSearcher searcher = new IndexSearcher(reader);
            try (BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in))) {
                final QueryParser queryParser = new QueryParser("all", new StandardAnalyzer());

                String line;
                while ((line = bufferedReader.readLine()) != null) {
                    Query query = queryParser.parse(line);
                    query = query.rewrite(reader);
                    System.out.println(searcher.count(query));
                }
            }
        }
    }
}
