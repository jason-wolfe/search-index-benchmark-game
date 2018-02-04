import com.eclipsesource.json.Json;
import com.eclipsesource.json.JsonObject;
import org.apache.lucene.document.Document;
import org.apache.lucene.document.LongPoint;
import org.apache.lucene.document.TextField;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.store.FSDirectory;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.StringReader;
import java.nio.file.Paths;

public class IndexStuff {
    public static void main(String[] args) throws IOException {
        IndexWriterConfig config = new IndexWriterConfig();
        IndexWriter writer = new IndexWriter(FSDirectory.open(Paths.get("/tmp/wiki_index_lucene")), config);

        final BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));

        final Document document = new Document();

        String line;
        while ((line = bufferedReader.readLine()) != null) {
            JsonObject parsed_doc = Json.parse(line).asObject();
            String url = parsed_doc.get("url").asString();

            String urlPrefix = "https://en.wikipedia.org/wiki?curid=";
            if (url.startsWith(urlPrefix)) {
                try {
                    long doc_id = Long.parseLong(url.substring(urlPrefix.length()));
                    String title = parsed_doc.get("title").asString();
                    String body = parsed_doc.get("body").asString();

                    document.clear();

                    document.add(new LongPoint("id", doc_id));
                    document.add(new TextField("title", new StringReader(title)));
                    document.add(new TextField("all", new StringReader(title + "\n" + body)));

                    long id = writer.addDocument(document);
                } catch (NumberFormatException e) {
                    continue;
                }
            }
        }

        writer.commit();
    }
}
