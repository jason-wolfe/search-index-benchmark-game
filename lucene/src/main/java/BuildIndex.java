import com.eclipsesource.json.Json;
import com.eclipsesource.json.JsonObject;
import org.apache.lucene.document.*;
import org.apache.lucene.index.IndexWriter;
import org.apache.lucene.index.IndexWriterConfig;
import org.apache.lucene.store.FSDirectory;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Paths;

public class BuildIndex {
    public static void main(String[] args) throws IOException {
        IndexWriterConfig config = new IndexWriterConfig();
        try (IndexWriter writer = new IndexWriter(FSDirectory.open(Paths.get("/tmp/wiki_index_lucene")), config)) {
            try (BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in))) {

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
                            document.add(new StoredField("id", doc_id));
                            document.add(new TextField("title", title, Field.Store.YES));
                            document.add(new TextField("all", title + "\n" + body, Field.Store.NO));

                            long id = writer.addDocument(document);
                        } catch (NumberFormatException e) {
                            continue;
                        }
                    }
                }
            }

            writer.commit();
        }
    }
}
