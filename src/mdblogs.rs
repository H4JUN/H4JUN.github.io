use phf::phf_map;

pub struct MarkdownBlog<'b> {
pub title: &'b str,
pub date_created: &'b str,
pub post: &'b str,
}

pub static BLOGS: phf::Map<&'static str, MarkdownBlog<'_>> = phf_map! {
    
    "R Programming Language Data Analysis Tutorial" => MarkdownBlog {
        title: "R Programming Language Data Analysis Tutorial",
        date_created: "2023-05-08",
        post: r#####"<p>This is my tutorial for R data analysis. It was one of my first analysis project done with R.</p>
<p>The original post can be found on my <a href="https://www.kaggle.com/code/ihazun01/analyzing-disasters-dataset">Kaggle</a>.
Please refer to the dataset on Kaggle.</p>
<!--more-->
<h1>Data Analysis</h1>
<h2>- Loading Packages</h2>
<p>Let's load the packages that we want to use using <code>library()</code>:</p>
<pre><code class="language-r">library(tidyverse) 
library(ggplot2)
</code></pre>
<p>If you are using R for the first time, packages might not be installed.
You can do so by calling <code>install.packages()</code>.</p>
<h2>- Loading Dataset</h2>
<p>Using R's base functions, we can read in csv files:</p>
<pre><code class="language-r">data &lt;- read.csv(&quot;../input/disasters-emdat/Disaster2021.csv&quot;)
</code></pre>
<p>However, the base R's <code>read.csv()</code> is troublesome as some columns might have broken character encoding.(Trust me, you don't want to spend your time on fixing the column names just so that you could call them correctly!)</p>
<p>Instead, we will use:</p>
<pre><code class="language-r">data &lt;- read_csv(&quot;../input/disasters-emdat/Disaster2021.csv&quot;)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## 
## ── Column specification ────────────────────────────────────────────────────────
## cols(
##   .default = col_character(),
##   Year = col_double(),
##   Seq = col_double(),
##   Aid_Contribution = col_logical(),
##   Dis_Mag_Value = col_double(),
##   Start_Year = col_double(),
##   Start_Month = col_double(),
##   Start_Day = col_double(),
##   End_Year = col_double(),
##   End_Month = col_double(),
##   End_Day = col_double(),
##   Total_Deaths = col_double(),
##   No_Injured = col_double(),
##   No_Affected = col_double(),
##   No_Homeless = col_double(),
##   Total_Affected = col_double(),
##   Recon_Costs = col_logical(),
##   Insured_Damages = col_double(),
##   Total_Damages = col_double(),
##   CPI = col_double()
## )
## ℹ Use `spec()` for the full column specifications.
</code></pre>
<p><code>read_csv()</code> is included in the <code>library(tidyverse)</code>. I think this is safer to read in .csv files.</p>
<hr />
<h1>Exploring dataset</h1>
<pre><code class="language-r">str(data)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## spec_tbl_df[,43] [15,901 × 43] (S3: spec_tbl_df/tbl_df/tbl/data.frame)
##  $ ID_No              : chr [1:15901] &quot;1900-9002-CPV&quot; &quot;1900-9001-IND&quot; &quot;1902-0012-GTM&quot; &quot;1902-0003-GTM&quot; ...
##  $ Year               : num [1:15901] 1900 1900 1902 1902 1902 ...
##  $ Seq                : num [1:15901] 9002 9001 12 3 10 ...
##  $ Disaster_Group     : chr [1:15901] &quot;Natural&quot; &quot;Natural&quot; &quot;Natural&quot; &quot;Natural&quot; ...
##  $ Disaster_Subgroup  : chr [1:15901] &quot;Climatological&quot; &quot;Climatological&quot; &quot;Geophysical&quot; &quot;Geophysical&quot; ...
##  $ Disaster_Type      : chr [1:15901] &quot;Drought&quot; &quot;Drought&quot; &quot;Earthquake&quot; &quot;Volcanic activity&quot; ...
##  $ Disaster_Subtype   : chr [1:15901] &quot;Drought&quot; &quot;Drought&quot; &quot;Ground movement&quot; &quot;Ash fall&quot; ...
##  $ Disaster_Subsubtype: chr [1:15901] NA NA NA NA ...
##  $ Event_Name         : chr [1:15901] NA NA NA &quot;Santa Maria&quot; ...
##  $ Entry_Criteria     : chr [1:15901] NA NA &quot;Kill&quot; &quot;Kill&quot; ...
##  $ Country            : chr [1:15901] &quot;Cabo Verde&quot; &quot;India&quot; &quot;Guatemala&quot; &quot;Guatemala&quot; ...
##  $ ISO                : chr [1:15901] &quot;CPV&quot; &quot;IND&quot; &quot;GTM&quot; &quot;GTM&quot; ...
##  $ Region             : chr [1:15901] &quot;Western Africa&quot; &quot;Southern Asia&quot; &quot;Central America&quot; &quot;Central America&quot; ...
##  $ Continent          : chr [1:15901] &quot;Africa&quot; &quot;Asia&quot; &quot;Americas&quot; &quot;Americas&quot; ...
##  $ Location           : chr [1:15901] &quot;Countrywide&quot; &quot;Bengal&quot; &quot;Quezaltenango, San Marcos&quot; NA ...
##  $ Origin             : chr [1:15901] NA NA NA NA ...
##  $ Associated_Dis     : chr [1:15901] &quot;Famine&quot; NA &quot;Tsunami/Tidal wave&quot; NA ...
##  $ Associated_Dis2    : chr [1:15901] NA NA NA NA ...
##  $ OFDA_Response      : chr [1:15901] NA NA NA NA ...
##  $ Appeal             : chr [1:15901] &quot;No&quot; &quot;No&quot; NA NA ...
##  $ Declaration        : chr [1:15901] &quot;No&quot; &quot;No&quot; NA NA ...
##  $ Aid_Contribution   : logi [1:15901] NA NA NA NA NA NA ...
##  $ Dis_Mag_Value      : num [1:15901] NA NA 8 NA NA NA NA NA NA 8 ...
...
</code></pre>
<p><code>str()</code> is a way to have an overview of the data. It specifies the columns and data types.</p>
<p>You can also use <code>glimpse()</code>.</p>
<pre><code class="language-r">glimpse(data)
</code></pre>
<p><code>head()</code> is also a popular way to have a closer look at the dataframe.</p>
<pre><code class="language-r">head(data) #Useful when reading the first few rows of the dataset
</code></pre>
<pre><code class="language-r">## # A tibble: 6 x 43
##   ID_No          Year   Seq Disaster_Group Disaster_Subgroup Disaster_Type      
##   &lt;chr&gt;         &lt;dbl&gt; &lt;dbl&gt; &lt;chr&gt;          &lt;chr&gt;             &lt;chr&gt;              
## 1 1900-9002-CPV  1900  9002 Natural        Climatological    Drought            
## 2 1900-9001-IND  1900  9001 Natural        Climatological    Drought            
## 3 1902-0012-GTM  1902    12 Natural        Geophysical       Earthquake         
## 4 1902-0003-GTM  1902     3 Natural        Geophysical       Volcanic activity  
## 5 1902-0010-GTM  1902    10 Natural        Geophysical       Volcanic activity  
## 6 1903-0006-CAN  1903     6 Natural        Geophysical       Mass movement (dry)
## # … with 37 more variables: Disaster_Subtype &lt;chr&gt;, Disaster_Subsubtype &lt;chr&gt;,
## #   Event_Name &lt;chr&gt;, Entry_Criteria &lt;chr&gt;, Country &lt;chr&gt;, ISO &lt;chr&gt;,
## #   Region &lt;chr&gt;, Continent &lt;chr&gt;, Location &lt;chr&gt;, Origin &lt;chr&gt;,
## #   Associated_Dis &lt;chr&gt;, Associated_Dis2 &lt;chr&gt;, OFDA_Response &lt;chr&gt;,
## #   Appeal &lt;chr&gt;, Declaration &lt;chr&gt;, Aid_Contribution &lt;lgl&gt;,
## #   Dis_Mag_Value &lt;dbl&gt;, Dis_Mag_Scale &lt;chr&gt;, Latitude &lt;chr&gt;, Longitude &lt;chr&gt;,
## #   Local_Time &lt;chr&gt;, River_Basin &lt;chr&gt;, Start_Year &lt;dbl&gt;, Start_Month &lt;dbl&gt;,
## #   Start_Day &lt;dbl&gt;, End_Year &lt;dbl&gt;, End_Month &lt;dbl&gt;, End_Day &lt;dbl&gt;,
## #   Total_Deaths &lt;dbl&gt;, No_Injured &lt;dbl&gt;, No_Affected &lt;dbl&gt;, No_Homeless &lt;dbl&gt;,
## #   Total_Affected &lt;dbl&gt;, Recon_Costs &lt;lgl&gt;, Insured_Damages &lt;dbl&gt;,
## #   Total_Damages &lt;dbl&gt;, CPI &lt;dbl&gt;
</code></pre>
<hr />
<h1>Accessing Columns and Rows</h1>
<p>To access the columns, we could use <code>$</code> or <code>[]</code>; however, the output is slightly different. The example below will explain better:</p>
<pre><code class="language-r">data[&quot;Year&quot;] #Using the name of the column
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 15,901 x 1
##     Year
##    &lt;dbl&gt;
##  1  1900
##  2  1900
##  3  1902
##  4  1902
##  5  1902
##  6  1903
##  7  1903
##  8  1904
##  9  1905
## 10  1905
## # … with 15,891 more rows
</code></pre>
<hr />
<pre><code class="language-r">data[,&quot;Year&quot;] #Using &quot;,&quot; to note that the first part is for rows and the second part is for columns. 
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 15,901 x 1
##     Year
##    &lt;dbl&gt;
##  1  1900
##  2  1900
##  3  1902
##  4  1902
##  5  1902
##  6  1903
##  7  1903
##  8  1904
##  9  1905
## 10  1905
## # … with 15,891 more rows
</code></pre>
<hr />
<pre><code class="language-r">data[1] #Using the index of the column
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 15,901 x 1
##    ID_No        
##    &lt;chr&gt;        
##  1 1900-9002-CPV
##  2 1900-9001-IND
##  3 1902-0012-GTM
##  4 1902-0003-GTM
##  5 1902-0010-GTM
##  6 1903-0006-CAN
##  7 1903-0012-COM
##  8 1904-0003-BGD
##  9 1905-0005-CAN
## 10 1905-0003-IND
## # … with 15,891 more rows
</code></pre>
<p>Let's take a look at the type or <code>class</code> of the data that we have extracted:</p>
<pre><code class="language-r">class(data[&quot;Year&quot;])
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## [1] &quot;tbl_df&quot;     &quot;tbl&quot;        &quot;data.frame&quot;
</code></pre>
<p>It's a dataframe.</p>
<p>Whereas using <code>$</code> will return a vector of type numeric in this case (Since <code>Year</code> column is full of numeric values of years).</p>
<pre><code class="language-r">head(data$Year)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## [1] 1900 1900 1902 1902 1902 1903
</code></pre>
<hr />
<pre><code class="language-r">class(data$Year)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## [1] &quot;numeric&quot;
</code></pre>
<p>So you might really need to pay attention to what you want to use.
You might call a function that applies only to vectors and not dataframes. Just know that <code>data[&quot;Year&quot;]</code> and <code>data$Year</code> look similar but they are different.</p>
<p>To access rows, we can do:</p>
<pre><code class="language-r">data[1,] #To access the first row
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 1 x 43
##   ID_No          Year   Seq Disaster_Group Disaster_Subgroup Disaster_Type
##   &lt;chr&gt;         &lt;dbl&gt; &lt;dbl&gt; &lt;chr&gt;          &lt;chr&gt;             &lt;chr&gt;        
## 1 1900-9002-CPV  1900  9002 Natural        Climatological    Drought      
## # … with 37 more variables: Disaster_Subtype &lt;chr&gt;, Disaster_Subsubtype &lt;chr&gt;,
## #   Event_Name &lt;chr&gt;, Entry_Criteria &lt;chr&gt;, Country &lt;chr&gt;, ISO &lt;chr&gt;,
## #   Region &lt;chr&gt;, Continent &lt;chr&gt;, Location &lt;chr&gt;, Origin &lt;chr&gt;,
## #   Associated_Dis &lt;chr&gt;, Associated_Dis2 &lt;chr&gt;, OFDA_Response &lt;chr&gt;,
## #   Appeal &lt;chr&gt;, Declaration &lt;chr&gt;, Aid_Contribution &lt;lgl&gt;,
## #   Dis_Mag_Value &lt;dbl&gt;, Dis_Mag_Scale &lt;chr&gt;, Latitude &lt;chr&gt;, Longitude &lt;chr&gt;,
## #   Local_Time &lt;chr&gt;, River_Basin &lt;chr&gt;, Start_Year &lt;dbl&gt;, Start_Month &lt;dbl&gt;,
## #   Start_Day &lt;dbl&gt;, End_Year &lt;dbl&gt;, End_Month &lt;dbl&gt;, End_Day &lt;dbl&gt;,
## #   Total_Deaths &lt;dbl&gt;, No_Injured &lt;dbl&gt;, No_Affected &lt;dbl&gt;, No_Homeless &lt;dbl&gt;,
## #   Total_Affected &lt;dbl&gt;, Recon_Costs &lt;lgl&gt;, Insured_Damages &lt;dbl&gt;,
## #   Total_Damages &lt;dbl&gt;, CPI &lt;dbl&gt;
</code></pre>
<p>Yes, <code>0</code> is not the starting index in R. The first row is <code>1</code>.
You might find the <code>,</code> irritating at first but without that you are performing column access <code>data[1]</code>(Look at the example of accessing columns).
I personally think it's clearer to put a <code>,</code> showing where the row and column parts are.</p>
<p><strong>Boolean Indexing</strong> is important. It opens so much capability to explore datasets. To retrieve a boolean vector of wehther the row belongs to Africa continent:</p>
<pre><code class="language-r">head(data$Continent == &quot;Africa&quot;)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## [1]  TRUE FALSE FALSE FALSE FALSE FALSE
</code></pre>
<p>This returns a vector of <code>TRUE</code> and <code>FALSE</code>. Why is this important?
Well, this allows us to get the rows that have <code>Africa</code> as their <code>Continent</code>.
Put the boolean vector inside <code>[]</code> of the <code>data</code>.</p>
<pre><code class="language-r">data[data$Continent == &quot;Africa&quot;]
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## Error: Must subset columns with a valid subscript vector.
## ℹ Logical subscripts must match the size of the indexed input.
## ✖ Input has size 43 but subscript `data$Continent == &quot;Africa&quot;` has size 15901.
</code></pre>
<p>Oops, looks like it didn't work! Yes, because we are using the boolean array to access columns and not rows. (Remember, the <code>,</code>!!!)</p>
<pre><code class="language-r">data[data$Continent == &quot;Africa&quot;, ]
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 2,908 x 43
##    ID_No          Year   Seq Disaster_Group Disaster_Subgroup Disaster_Type    
##    &lt;chr&gt;         &lt;dbl&gt; &lt;dbl&gt; &lt;chr&gt;          &lt;chr&gt;             &lt;chr&gt;            
##  1 1900-9002-CPV  1900  9002 Natural        Climatological    Drought          
##  2 1903-0012-COM  1903    12 Natural        Geophysical       Volcanic activity
##  3 1910-9006-BFA  1910  9006 Natural        Climatological    Drought          
##  4 1910-9006-CPV  1910  9006 Natural        Climatological    Drought          
##  5 1910-0005-DZA  1910     5 Natural        Geophysical       Earthquake       
##  6 1910-9006-GMB  1910  9006 Natural        Climatological    Drought          
##  7 1910-9006-GNB  1910  9006 Natural        Climatological    Drought          
##  8 1920-9004-CPV  1920  9004 Natural        Climatological    Drought          
##  9 1926-0014-EGY  1926    14 Natural        Geophysical       Earthquake       
## 10 1927-0012-DZA  1927    12 Natural        Hydrological      Flood            
## # … with 2,898 more rows, and 37 more variables: Disaster_Subtype &lt;chr&gt;,
## #   Disaster_Subsubtype &lt;chr&gt;, Event_Name &lt;chr&gt;, Entry_Criteria &lt;chr&gt;,
## #   Country &lt;chr&gt;, ISO &lt;chr&gt;, Region &lt;chr&gt;, Continent &lt;chr&gt;, Location &lt;chr&gt;,
## #   Origin &lt;chr&gt;, Associated_Dis &lt;chr&gt;, Associated_Dis2 &lt;chr&gt;,
## #   OFDA_Response &lt;chr&gt;, Appeal &lt;chr&gt;, Declaration &lt;chr&gt;,
## #   Aid_Contribution &lt;lgl&gt;, Dis_Mag_Value &lt;dbl&gt;, Dis_Mag_Scale &lt;chr&gt;,
## #   Latitude &lt;chr&gt;, Longitude &lt;chr&gt;, Local_Time &lt;chr&gt;, River_Basin &lt;chr&gt;,
## #   Start_Year &lt;dbl&gt;, Start_Month &lt;dbl&gt;, Start_Day &lt;dbl&gt;, End_Year &lt;dbl&gt;,
## #   End_Month &lt;dbl&gt;, End_Day &lt;dbl&gt;, Total_Deaths &lt;dbl&gt;, No_Injured &lt;dbl&gt;,
## #   No_Affected &lt;dbl&gt;, No_Homeless &lt;dbl&gt;, Total_Affected &lt;dbl&gt;,
## #   Recon_Costs &lt;lgl&gt;, Insured_Damages &lt;dbl&gt;, Total_Damages &lt;dbl&gt;, CPI &lt;dbl&gt;
</code></pre>
<p>We have gained access to only the rows that have <code>Africa</code> as their <code>Continent</code>.</p>
<p>If you are like me who has to know every single thing going on before your fingers start to code and this concept of boolean indexing doesn't make sense to you, then let's think about it:
We accessed the column <code>Continent</code> and retrieved it as an array using <code>$</code>. Then we checked if the item in that array is <code>Africa</code>. This returns a boolean array of the length equivalent to:</p>
<pre><code class="language-r">length(data$Continent == &quot;Africa&quot;)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## [1] 15901
</code></pre>
<p>which should be equivalent to the number of rows of the dataframe since it's originally extracted from there,</p>
<pre><code class="language-r">length(rownames(data))
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## [1] 15901
</code></pre>
<p>Having the same length in the rows, we could use this boolean array as the criteria to filter out rows marked as <code>TRUE</code> in the dataframe.</p>
<p>Some other useful things:</p>
<pre><code class="language-r">subset(data, criteria ) #Subsetting from the dataframe using the criteria
x %in% y #Similar to in in python. Returns boolean datatype if x is in y
ifelse(test, if TRUE, if FALSE) #It is a way to run an if statement and return values accordingly in a single line
</code></pre>
<hr />
<h1>Manipulating Data</h1>
<p>Let's extract the important disasters that we want:</p>
<pre><code class="language-r">fiveDis &lt;- subset(data, Disaster_Type %in% c(&quot;Earthquake&quot;, &quot;Drought&quot;, &quot;Storm&quot;, &quot;Flood&quot;, &quot;Extreme temperature&quot;))
</code></pre>
<p>What just happened? Well, first of all, <code>subset()</code> returns a subset of the dataframe using the criteria.
The criteria in our case is <code>Disaster_Type %in% c(,,,)</code>.
<code>Disaster_Type</code> is a column in the dataframe.</p>
<p><strong>Wait</strong>, shouldn't we use <code>data$Disaster_Type</code> to access that column?</p>
<p>Yes, but in <code>subset()</code>, we have already defined the data from which we are subsetting. Having defined what the original data is, we can just use the column name without the need to call the main data.</p>
<p><code>c()</code> is R's way of making vectors. In our example, we have combined the five earthquakes' names in our vector using <code>c()</code>. Then we filtered out the disasters that belonged to these categories.</p>
<p>Now that we have the disasters that we need in <code>fiveDis</code>, let's get the continents:</p>
<pre><code class="language-r">Africa &lt;- fiveDis[fiveDis$Continent == &quot;Africa&quot;, ]
</code></pre>
<p>It's a good practice to only keep the dataframe with least amount of columns that we need to use in our analysis:</p>
<pre><code class="language-r">#Columns that we need:
colNeeded = c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)
Africa &lt;- Africa[colNeeded]
</code></pre>
<p>Let's take a look at our data before moving on:</p>
<pre><code class="language-r">head(Africa)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 6 x 6
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   &lt;chr&gt;     &lt;dbl&gt; &lt;chr&gt;                 &lt;dbl&gt;        &lt;dbl&gt;          &lt;dbl&gt;
## 1 Africa     1900 Drought                  NA        11000             NA
## 2 Africa     1910 Drought                  NA           NA             NA
## 3 Africa     1910 Drought                  NA           NA             NA
## 4 Africa     1910 Earthquake               NA           12             NA
## 5 Africa     1910 Drought                  NA           NA             NA
## 6 Africa     1910 Drought                  NA           NA             NA
</code></pre>
<p>It looks like we have so much NA values.
To solve this, we can first locate which rows have NA values and then replace the NA to something else in that row.
NA's are found in <code>Total_Damages</code>, <code>Total_Deaths</code>, and <code>Total_Affected</code>.</p>
<p>Use <code>is.na()</code> to get boolean array of whether the column has NA values.</p>
<pre><code class="language-r">is.na(Africa$Total_Affected) #Returns boolean array. Use it in indexing
Africa[is.na(Africa$Total_Affected), ] #Now we have access to dataframe where Total_Affected column in this dataframe is full of NA values
Africa[is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] #Now we have access to the column!
</code></pre>
<p>Now that we have access, what should we assign the values?
I personally thought that <code>median</code> values of disasters in that continent would be appropriate rather than <code>mean</code> values since disasters have outliers and mean values might heavily be impacted on the presence of outliers.</p>
<p>R comes from statistical background. It already has <code>mean()</code>, <code>median()</code>, <code>sd()</code> for standard deviation, <code>var()</code> for variance in samples, and so on.
Let's assign the median value of <code>Total_Affected</code> to each NA values in <code>Total_Affected</code> columns:</p>
<pre><code class="language-r">Africa[is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa$Total_Affected)
</code></pre>
<p>It seems that there is no error, but our dataset does not look okay:</p>
<pre><code class="language-r">head(Africa)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 6 x 6
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   &lt;chr&gt;     &lt;dbl&gt; &lt;chr&gt;                 &lt;dbl&gt;        &lt;dbl&gt;          &lt;dbl&gt;
## 1 Africa     1900 Drought                  NA        11000             NA
## 2 Africa     1910 Drought                  NA           NA             NA
## 3 Africa     1910 Drought                  NA           NA             NA
## 4 Africa     1910 Earthquake               NA           12             NA
## 5 Africa     1910 Drought                  NA           NA             NA
## 6 Africa     1910 Drought                  NA           NA             NA
</code></pre>
<p>That's because we had NA values in the main <code>Africa</code> dataset. What's the median value of <code>5, 6, 7, NA</code>? The answer is not clear. We have to remove the NA values while calculating for <code>median()</code>. We add the following flag <code>na.rm = TRUE</code>:</p>
<pre><code class="language-r">Africa[is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa$Total_Affected, na.rm = TRUE)
</code></pre>
<p>Now the data looks fine. If you argue that each disaster group must have its own median value of <code>Total_Affected</code> values, I would agree. We could do something like:</p>
<pre><code class="language-r">Africa[(Africa$Disaster_Type == &quot;Storm&quot;) &amp; is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Storm&quot;, ]$Total_Affected, na.rm = TRUE)
</code></pre>
<p>The first part is finding the data where <code>Disaster_Type</code> is <code>Storm</code> and has NA values in the <code>Total_Affected</code> column, then assigning it with the median value of <code>Total_Affected</code> column where <code>Disaster_Type</code> is <code>Storm</code>.</p>
<p>Applying this to all the Disasters:</p>
<pre><code class="language-r">Africa &lt;- fiveDis[fiveDis$Continent == &quot;Africa&quot;, c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)]

Africa[(Africa$Disaster_Type == &quot;Drought&quot;) &amp; is.na(Africa$Total_Damages), &quot;Total_Damages&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Drought&quot;, ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Earthquake&quot;) &amp; is.na(Africa$Total_Damages), &quot;Total_Damages&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Earthquake&quot;, ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Extreme temperature&quot;) &amp; is.na(Africa$Total_Damages), &quot;Total_Damages&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Extreme temperature&quot;, ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Flood&quot;) &amp; is.na(Africa$Total_Damages), &quot;Total_Damages&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Flood&quot;, ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Storm&quot;) &amp; is.na(Africa$Total_Damages), &quot;Total_Damages&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Storm&quot;, ]$Total_Damages, na.rm = TRUE)

Africa[(Africa$Disaster_Type == &quot;Drought&quot;) &amp; is.na(Africa$Total_Deaths), &quot;Total_Deaths&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Drought&quot;, ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Earthquake&quot;) &amp; is.na(Africa$Total_Deaths), &quot;Total_Deaths&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Earthquake&quot;, ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Extreme temperature&quot;) &amp; is.na(Africa$Total_Deaths), &quot;Total_Deaths&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Extreme temperature&quot;, ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Flood&quot;) &amp; is.na(Africa$Total_Deaths), &quot;Total_Deaths&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Flood&quot;, ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Storm&quot;) &amp; is.na(Africa$Total_Deaths), &quot;Total_Deaths&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Storm&quot;, ]$Total_Deaths, na.rm = TRUE)

Africa[(Africa$Disaster_Type == &quot;Drought&quot;) &amp; is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Drought&quot;, ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Earthquake&quot;) &amp; is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Earthquake&quot;, ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Extreme temperature&quot;) &amp; is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Extreme temperature&quot;, ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Flood&quot;) &amp; is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Flood&quot;, ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == &quot;Storm&quot;) &amp; is.na(Africa$Total_Affected), &quot;Total_Affected&quot;] &lt;- median(Africa[Africa$Disaster_Type == &quot;Storm&quot;, ]$Total_Affected, na.rm = TRUE)
</code></pre>
<p>It looks messy! But for beginners, this is the best you could make out of base R, and I think it's important to understand the basics before moving on to higher level.</p>
<p>We have 5 Continents in total. Copying and pasting the above code for 5 continents is not so efficient, yet there is no other way for us now. At least that is what I did back when base R was the only thing I knew.</p>
<hr />
<h1><strong>dplyr</strong> in <strong>tidyverse</strong> package</h1>
<p>For those experts in R who have long waited for me to start talking about <code>dplyr</code>, I can't believe you stayed with me so far (Although I'm pretty sure many would have left when they saw how inefficient I was).</p>
<p>For all the beginners, would you believe if the above code could be simplified to:</p>
<pre><code class="language-r">Africa &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Africa&quot;) %&gt;%
  select(c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)) %&gt;%
  group_by(Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()
</code></pre>
<p><em>New things:</em>
To start with, <code>%&gt;%</code> is called piping. Simply put, what it does is take whatever output we get before the <code>%&gt;%</code> operator and put it as an input of what comes after the operator.</p>
<p><code>filter()</code> is choosing the rows that meet certain criteria</p>
<p><code>select()</code> is choosing the columns with the names given</p>
<p><code>group_by()</code> is making partitions inside the dataframe, so that we could assess the groups individually</p>
<p><code>mutate()</code> is modifying any existing column or making a new one</p>
<p><code>ungroup()</code> is ungrouping groups made previously(We do this in case we get unexpected results that came out of assessing each groups individually when we wanted to assess the whole dataframe)</p>
<p>Breaking down the code:</p>
<pre><code class="language-r">fiveDis %&gt;%
  filter(Continent == &quot;Africa&quot;) %&gt;%
#Equivalent to fiveDis[fiveDis$Continent == &quot;Africa&quot;, ]
</code></pre>
<p>Note that without the <code>%&gt;%</code>, we would have needed to insert <code>fiveDis</code> inside the <code>filter()</code>:</p>
<pre><code class="language-r">filter(fiveDis, Continent == &quot;Africa&quot;)
</code></pre>
<p>Moving on, we use the newly acquired &quot;filtered&quot; dataframe and use it as an input for the next verb <code>select()</code>:</p>
<pre><code class="language-r">select(c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)) %&gt;%
</code></pre>
<p>We select the columns that we want, combine them as a list using <code>c()</code>, and then use it as the criteria to select the columns.</p>
<p>Next, we will group each <code>Disaster_Type</code> as a partition.</p>
<pre><code class="language-r">group_by(Disaster_Type) %&gt;%
</code></pre>
<p>The concept of grouping might seem esoteric at first, but it's useful.
<code>group_by</code> does not modify the dataframe itself and I mean by modifications such as sorting and subsetting:</p>
<pre><code class="language-r">Africa &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Africa&quot;) %&gt;%
  select(c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)) %&gt;%
  group_by(Disaster_Type)
head(Africa)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 6 x 6
## # Groups:   Disaster_Type [2]
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   &lt;chr&gt;     &lt;dbl&gt; &lt;chr&gt;                 &lt;dbl&gt;        &lt;dbl&gt;          &lt;dbl&gt;
## 1 Africa     1900 Drought                  NA        11000             NA
## 2 Africa     1910 Drought                  NA           NA             NA
## 3 Africa     1910 Drought                  NA           NA             NA
## 4 Africa     1910 Earthquake               NA           12             NA
## 5 Africa     1910 Drought                  NA           NA             NA
## 6 Africa     1910 Drought                  NA           NA             NA
</code></pre>
<p>What <code>group_by()</code> does is adding a new information in the dataframe called <code>Groups</code> as shown in the code. We can see that <code># Groups: Disaster_Type</code>. This will allow us to do statistical operations on each partition of <code>Disaster_Type</code>.</p>
<p>Let's see how <code>group_by()</code> helps in <code>mutate()</code>:</p>
<pre><code class="language-r">mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
</code></pre>
<p>So, <code>mutate()</code> allows modification or addition of new columns. Looking at our first statement, we are modifying <code>Total_Damages</code> column. we test if <code>Total_Damages</code> have NA values. If it does, then assign to the NA value, the median of the <code>Total_Damages</code>.
Is it assigning the median of all the values of <code>Total_Damages</code> in the <code>Africa</code> dataframe? No, here is where <code>group_by()</code> kicks in. Since we have already &quot;partitioned&quot; the dataframe into groups of disasters, when we say apply <code>median()</code> on this column, it will look for the individual groups/partitions to apply and assign values correspondingly. Again, <code>na.rm=TRUE</code> is to remove NA values since with the NA values included, we can't calculate <code>mean()</code> or <code>median()</code>.</p>
<p>If it's not NA value, then just assign it's own value.</p>
<p>One more thing to note, we have said that <code>mutate()</code> allows the making a new column. You can see this in the last statement where we made a new column <code>Total_Casualty</code> which is equal to the sum of <code>Total_Affected</code> and <code>Total_Deaths</code>.</p>
<p><code>ungroup()</code> is used to free the groups/partitions.</p>
<p>Our data looks like this after cleaning:</p>
<pre><code class="language-r">Africa &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Africa&quot;) %&gt;%
  select(c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)) %&gt;%
  group_by(Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()
</code></pre>
<pre><code class="language-r">Africa
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 1,843 x 7
##    Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##    &lt;chr&gt;     &lt;dbl&gt; &lt;chr&gt;                 &lt;dbl&gt;        &lt;dbl&gt;          &lt;dbl&gt;
##  1 Africa     1900 Drought               34000        11000         838000
##  2 Africa     1910 Drought               34000          237         838000
##  3 Africa     1910 Drought               34000          237         838000
##  4 Africa     1910 Earthquake            28000           12           2000
##  5 Africa     1910 Drought               34000          237         838000
##  6 Africa     1910 Drought               34000          237         838000
##  7 Africa     1920 Drought               34000        24000         838000
##  8 Africa     1926 Earthquake            28000           12           2000
##  9 Africa     1927 Flood                 10000         3000          12000
## 10 Africa     1939 Earthquake            28000           22           7630
## # … with 1,833 more rows, and 1 more variable: Total_Casualty &lt;dbl&gt;
</code></pre>
<p>What if I want to have a dataframe that summarizes the influence of disasters per disaster type? Somethinng like, the average death by x disaster is y in z continent?</p>
<p>Have a look at the following code:</p>
<pre><code class="language-r">#Average data for Africa Dataset
Africa_Avg &lt;- Africa %&gt;%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %&gt;%
  group_by(Disaster_Type) %&gt;%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n())
</code></pre>
<p>We know what <code>select()</code> and <code>group_by()</code> is.</p>
<p><code>summarize()</code> is a useful verb to apply a statistical function on a dataframe and get the result. It returns the summary of the dataframe. Inside <code>summarize()</code>, you'll need to include the new column names which you want to store the summarized data. I like to think of it as <code>mutate()</code>, making new columns with newly assigned data.</p>
<p><code>n()</code> is a way to get the count of items in a group. As far as I know, <code>n()</code> only works inside <code>mutate()</code> and <code>summarize()</code> when we have defined the groups.</p>
<p>Where's the <code>ungroup()</code>? <code>summarize()</code> takes off or &quot;peels&quot; off one group after it is performed. Since we only have one group <code>group_by(Disaster_Type)</code>, <code>summarize()</code> will leave us with no groups.</p>
<p>Let's see what happens:</p>
<pre><code class="language-r">head(Africa_Avg)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 5 x 4
##   Disaster_Type       mean_Damage mean_Casualty count
##   &lt;chr&gt;                     &lt;dbl&gt;         &lt;dbl&gt; &lt;int&gt;
## 1 Drought                  49664.      1562370.   335
## 2 Earthquake              192743.        27539.    74
## 3 Extreme temperature      23904.       242663.    20
## 4 Flood                    17143.        80775.  1128
## 5 Storm                    39436.        83420.   286
</code></pre>
<p>That looks cleaner!</p>
<p>Let's apply this to all the continents:</p>
<pre><code class="language-r">#Africa Data:
Africa &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Africa&quot;) %&gt;%
  select(c(&quot;Continent&quot;, &quot;Year&quot;, &quot;Disaster_Type&quot;, &quot;Total_Damages&quot;, &quot;Total_Deaths&quot;, &quot;Total_Affected&quot;)) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()

#Average data for Africa Dataset
Africa_Avg &lt;- Africa %&gt;%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %&gt;%
  ungroup()

#America Data
America &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Americas&quot;) %&gt;%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()

America_Avg &lt;- America %&gt;%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %&gt;%
  ungroup()

#Asia Data
Asia &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Asia&quot;) %&gt;%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()

Asia_Avg &lt;- Asia %&gt;%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %&gt;%
  ungroup()

#Europe Data
Europe &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Europe&quot;) %&gt;%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()

Europe_Avg &lt;- Europe %&gt;%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %&gt;%
  ungroup()

#Oceania Data
Oceania &lt;- fiveDis %&gt;%
  filter(Continent == &quot;Oceania&quot;) %&gt;%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %&gt;%
  ungroup()

Oceania_Avg &lt;- Oceania %&gt;%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %&gt;%
  ungroup()
</code></pre>
<p>Before we move on to plotting, I would like to cover one more thing.
You might want to combine dataframes row-wise or column-wise according to what you want to do with your data.
<code>rbind()</code> and <code>cbind()</code> are useful to combine rows and columns.</p>
<p>If I want to make one dataframe out of the continents with the NA values removed:</p>
<pre><code class="language-r">fiveDis_narm &lt;- rbind(Africa, America, Asia, Europe, Oceania)
</code></pre>
<p>I could run the above code to do so.</p>
<p>Moving further, I could include how many counts are there for each <code>Continent</code>-<code>Disaster_Type</code> pairs using <code>n()</code>:</p>
<pre><code class="language-r">fiveDis_narm &lt;- rbind(Africa, America, Asia, Europe, Oceania) %&gt;%
  group_by(Continent, Disaster_Type) %&gt;%
  mutate(count = n()) %&gt;%
  ungroup()
</code></pre>
<pre><code class="language-r">head(fiveDis_narm)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 6 x 8
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   &lt;chr&gt;     &lt;dbl&gt; &lt;chr&gt;                 &lt;dbl&gt;        &lt;dbl&gt;          &lt;dbl&gt;
## 1 Africa     1900 Drought               34000        11000         838000
## 2 Africa     1910 Drought               34000          237         838000
## 3 Africa     1910 Drought               34000          237         838000
## 4 Africa     1910 Earthquake            28000           12           2000
## 5 Africa     1910 Drought               34000          237         838000
## 6 Africa     1910 Drought               34000          237         838000
## # … with 2 more variables: Total_Casualty &lt;dbl&gt;, count &lt;int&gt;
</code></pre>
<p>I could do the same thing for the Average values of disasters per continent:</p>
<pre><code class="language-r">continent_Avg &lt;- rbind(Africa_Avg, America_Avg, Asia_Avg, Europe_Avg, Oceania_Avg)
</code></pre>
<p>Alright, our dataframes are ready. Let's plot!</p>
<hr />
<h1>Plots using <code>ggplot</code></h1>
<p>Let's make a quick dataframe that counts the occurrences of disasters in each year in the dataset:</p>
<pre><code class="language-r">count_Year &lt;- data %&gt;%
  group_by(Year) %&gt;%
  summarize(count = n()) %&gt;%
  ungroup()
</code></pre>
<p>Simple and easy. It looks like this:</p>
<pre><code class="language-r">head(count_Year)
</code></pre>
<p>Output:</p>
<pre><code class="language-r">## # A tibble: 6 x 2
##    Year count
##   &lt;dbl&gt; &lt;int&gt;
## 1  1900     7
## 2  1901     2
## 3  1902    10
## 4  1903    12
## 5  1904     4
## 6  1905     8
</code></pre>
<p>Using this dataframe, we can plot the counts of disasters for each year.
<code>ggplot()</code> takes <code>data</code> and <code>aes()</code> to start drawing the &quot;skeleton&quot; of the plot. Consider this as the frame in which we will start drawing our plots by adding <code>layers</code> to it.</p>
<pre><code class="language-r">ggplot(data = count_Year, aes(x = Year, y = count))
</code></pre>
<p>Output:
<img src="/assets/img/R-Basics-plot-1.png" alt="ggplot" /></p>
<p>You might wonder why it did not show any drawing. That's because we mapped the variables/columns to <code>x</code> and <code>y</code> but did not specify what to draw with it. Am I telling it to draw a linechart? a scatterplot? a barplot?</p>
<p>We specify this by adding layers.
<code>geom_point()</code> adds points to the plot making it a scatterplot.</p>
<pre><code class="language-r">ggplot(data = count_Year, aes(x = Year, y = count)) + 
  geom_point()
</code></pre>
<p>Output:
<img src="/assets/img/R-Basics-plot-2.png" alt="ggplot" /></p>
<p>Don't forget to put <code>+</code> everytime you want to &quot;add&quot; a layer.</p>
<p>You might want to go and check for the documentation to see lots and lots of layers you could add to make your plots more insightful.</p>
<p>For example, you could do something like this:</p>
<pre><code class="language-r">ggplot(data = count_Year, aes(x = Year, y = count)) +
  geom_point() + #Scatterplot
  geom_text(aes(label = count), check_overlap = TRUE, vjust = -1) + #Labeling the counts
  scale_x_continuous(&quot;Year&quot;, breaks = c(1900, 1920, 1940, 1960, 1980, 1988, 2000, 2020), limits = c(1900, 2020))+ #Scaling the x-axis and giving it a label
  scale_y_continuous(&quot;Number of Disasters&quot;, breaks = c(0, 20, 50, 100, 200, 300, 400, 500), limits = c(0, 600)) +
  geom_vline(xintercept = 1988, linetype = &quot;dotted&quot;, color = &quot;red&quot;, size = 1) + #Adding a vertical line to the plot
  labs(title = &quot;Total Number of Disasters Occurring in Each Year&quot;,
        subtitle = &quot;Disasters start to occur more frequently after the year 1988, the year that marks the beginning of climate change&quot;)+ #Labelling title and subtitle
  theme_bw() + #Adding the overall theme to the plot
  theme(plot.title = element_text(face = &quot;bold&quot;, size = 17), 
        plot.subtitle = element_text(face = &quot;italic&quot;, size = 10)) + #Modifying the title and subtitle
  stat_smooth(se = FALSE, col = &quot;gray&quot;) #Adding smoothing line
</code></pre>
<p>Output:
<img src="/assets/img/R-Basics-plot-3.png" alt="ggplot" /></p>
<p>Looks like the number of disasters are increasing over time.</p>
<hr />
<p>Using the dataframes that we made previously, we could plot the following graphs:</p>
<pre><code class="language-r">ggplot(continent_Avg, aes(color = Disaster_Type)) +
      geom_point(aes(y = mean_Casualty / 1000, x = mean_Damage/ 1000, shape = Disaster_Type), alpha = 0.5, size = 5) +
      facet_wrap(~Continent, scale=&quot;free&quot;) +
      scale_shape_manual(values = c(15, 16, 17, 18, 20)) +
      scale_y_continuous(&quot;Average number of casualties in (,000) person&quot;, trans = &quot;log10&quot;) + 
      scale_x_continuous(&quot;Average damage in (,000,000) dollars&quot;, trans = &quot;log10&quot;) +
      scale_color_manual(values = c(&quot;#cc9900&quot;, &quot;#663300&quot;, &quot;#ff0000&quot;, &quot;#0066dd&quot;, &quot;#999999&quot;)) +
      labs(title = &quot;How critical are disasters in your continent?&quot;) +
      theme(plot.title = element_text(face = &quot;bold&quot;, size = 17))
</code></pre>
<p>Output:
<img src="/assets/img/R-Basics-plot-4.png" alt="ggplot" /></p>
<hr />
<pre><code class="language-r">ggplot(data = fiveDis, aes(x = Year, color = Disaster_Type)) +
    geom_line(stat = &quot;count&quot;, alpha = 0.6, size = 1)+
    scale_x_continuous(&quot;Year of Occurrence&quot;, breaks = c(1900, 1980, 1988, 2020), limits = c(1900, 2020)) +
    scale_y_continuous(&quot;Number of Disasters&quot;, breaks = c(0, 10, 30, 50, 100, 150, 200)) + 
    geom_vline(xintercept = 1988, linetype = &quot;dotted&quot;, color = &quot;red&quot;, size = 1) +
    theme_bw() +
    labs(title = &quot;Five Big Disasters in the World&quot;,
          subtitle = &quot;marking the year 1988, the year which faced the beginning of climate change&quot;) +
    theme(plot.title = element_text(face = &quot;bold&quot;, size = 17),
          plot.subtitle = element_text(face = &quot;italic&quot;, size = 10))
</code></pre>
<p>Output:
<img src="/assets/img/R-Basics-plot-5.png" alt="ggplot" /></p>
<hr />
<h1>Discovering R</h1>
<hr />
<h2>- <code>Shiny</code></h2>
<p>You might want your plots to be interactive, in a sense that you want to display a different plot whenever the audience selects something as an input. <code>Shiny</code> package is the one you want to use.</p>
<pre><code class="language-r">library(shiny)
</code></pre>
<p>Sadly, it looks like Kaggle's notebook script does not allow the usage of interactive shiny <code>renderplot()</code> function. It keeps throwing errors.</p>
<p>However, feel free to check out my implemetation of the Disasters data using Shiny <a href="https://ihazun01.shinyapps.io/Deploy/">here</a>.</p>
<hr />
<h2>- <code>wordcloud</code></h2>
<p><code>wordcloud</code> is a package that allows us to perform data analysis based on texts and words.</p>
<p>We could know what the most frequent disaster was in the year 2020:</p>
<pre><code class="language-r">library(wordcloud)

hotDis2020 &lt;- data %&gt;%
  filter(Year == 2020) %&gt;%
  group_by(Disaster_Type) %&gt;%
  summarize(count = n())
wordcloud(word = hotDis2020$Disaster_Type, freq = hotDis2020$count, color=&quot;Red&quot;)
</code></pre>
<p>Output:
<img src="/assets/img/R-Basics-plot-6.png" alt="ggplot" /></p>
<hr />
<h1>Conclusion</h1>
<hr />
<p>R opens up so many capabilities to analyze data and draw plots. If you are interested in any kind of plots, go look for it in the R documentation. I hope this tutorial and analysis stimulates you to explore and discover more of R!</p>
<hr />
<h2>Acknowledgements</h2>
<p>This dataset has been acquired from <a href="https://www.emdat.be/">EM-DAT</a> database. I hereby acknowledge the source of the dataset is EM-DAT. <strong><em><code>EM-DAT: The Emergency Events Database - Université catholique de Louvain (UCL) - CRED, D. Guha-Sapir - www.emdat.be, Brussels, Belgium.</code></em></strong></p>
<hr />
<p><strong><em><code>“Everyone has a plan until they get punched in the mouth.”</code></em></strong></p>"#####,
    },
        };