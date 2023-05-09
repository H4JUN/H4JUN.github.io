This is my tutorial for R data analysis. It was one of my first analysis project done with R.

The original post can be found on my [Kaggle](https://www.kaggle.com/code/ihazun01/analyzing-disasters-dataset).
Please refer to the dataset on Kaggle.

<!--more-->


# Data Analysis

## - Loading Packages
Let's load the packages that we want to use using `library()`:

```r
library(tidyverse) 
library(ggplot2)
```
If you are using R for the first time, packages might not be installed.
You can do so by calling `install.packages()`.

## - Loading Dataset
Using R's base functions, we can read in csv files:

```r
data <- read.csv("../input/disasters-emdat/Disaster2021.csv")
```
However, the base R's `read.csv()` is troublesome as some columns might have broken character encoding.(Trust me, you don't want to spend your time on fixing the column names just so that you could call them correctly!)

Instead, we will use:
```r
data <- read_csv("../input/disasters-emdat/Disaster2021.csv")
```
Output:
```r
## 
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
```
`read_csv()` is included in the `library(tidyverse)`. I think this is safer to read in .csv files.

***

# Exploring dataset
```r
str(data)
```
Output:
```r
## spec_tbl_df[,43] [15,901 × 43] (S3: spec_tbl_df/tbl_df/tbl/data.frame)
##  $ ID_No              : chr [1:15901] "1900-9002-CPV" "1900-9001-IND" "1902-0012-GTM" "1902-0003-GTM" ...
##  $ Year               : num [1:15901] 1900 1900 1902 1902 1902 ...
##  $ Seq                : num [1:15901] 9002 9001 12 3 10 ...
##  $ Disaster_Group     : chr [1:15901] "Natural" "Natural" "Natural" "Natural" ...
##  $ Disaster_Subgroup  : chr [1:15901] "Climatological" "Climatological" "Geophysical" "Geophysical" ...
##  $ Disaster_Type      : chr [1:15901] "Drought" "Drought" "Earthquake" "Volcanic activity" ...
##  $ Disaster_Subtype   : chr [1:15901] "Drought" "Drought" "Ground movement" "Ash fall" ...
##  $ Disaster_Subsubtype: chr [1:15901] NA NA NA NA ...
##  $ Event_Name         : chr [1:15901] NA NA NA "Santa Maria" ...
##  $ Entry_Criteria     : chr [1:15901] NA NA "Kill" "Kill" ...
##  $ Country            : chr [1:15901] "Cabo Verde" "India" "Guatemala" "Guatemala" ...
##  $ ISO                : chr [1:15901] "CPV" "IND" "GTM" "GTM" ...
##  $ Region             : chr [1:15901] "Western Africa" "Southern Asia" "Central America" "Central America" ...
##  $ Continent          : chr [1:15901] "Africa" "Asia" "Americas" "Americas" ...
##  $ Location           : chr [1:15901] "Countrywide" "Bengal" "Quezaltenango, San Marcos" NA ...
##  $ Origin             : chr [1:15901] NA NA NA NA ...
##  $ Associated_Dis     : chr [1:15901] "Famine" NA "Tsunami/Tidal wave" NA ...
##  $ Associated_Dis2    : chr [1:15901] NA NA NA NA ...
##  $ OFDA_Response      : chr [1:15901] NA NA NA NA ...
##  $ Appeal             : chr [1:15901] "No" "No" NA NA ...
##  $ Declaration        : chr [1:15901] "No" "No" NA NA ...
##  $ Aid_Contribution   : logi [1:15901] NA NA NA NA NA NA ...
##  $ Dis_Mag_Value      : num [1:15901] NA NA 8 NA NA NA NA NA NA 8 ...
...
```
`str()` is a way to have an overview of the data. It specifies the columns and data types.

You can also use `glimpse()`.
```r
glimpse(data)
```
`head()` is also a popular way to have a closer look at the dataframe.
```r
head(data) #Useful when reading the first few rows of the dataset
```
```r
## # A tibble: 6 x 43
##   ID_No          Year   Seq Disaster_Group Disaster_Subgroup Disaster_Type      
##   <chr>         <dbl> <dbl> <chr>          <chr>             <chr>              
## 1 1900-9002-CPV  1900  9002 Natural        Climatological    Drought            
## 2 1900-9001-IND  1900  9001 Natural        Climatological    Drought            
## 3 1902-0012-GTM  1902    12 Natural        Geophysical       Earthquake         
## 4 1902-0003-GTM  1902     3 Natural        Geophysical       Volcanic activity  
## 5 1902-0010-GTM  1902    10 Natural        Geophysical       Volcanic activity  
## 6 1903-0006-CAN  1903     6 Natural        Geophysical       Mass movement (dry)
## # … with 37 more variables: Disaster_Subtype <chr>, Disaster_Subsubtype <chr>,
## #   Event_Name <chr>, Entry_Criteria <chr>, Country <chr>, ISO <chr>,
## #   Region <chr>, Continent <chr>, Location <chr>, Origin <chr>,
## #   Associated_Dis <chr>, Associated_Dis2 <chr>, OFDA_Response <chr>,
## #   Appeal <chr>, Declaration <chr>, Aid_Contribution <lgl>,
## #   Dis_Mag_Value <dbl>, Dis_Mag_Scale <chr>, Latitude <chr>, Longitude <chr>,
## #   Local_Time <chr>, River_Basin <chr>, Start_Year <dbl>, Start_Month <dbl>,
## #   Start_Day <dbl>, End_Year <dbl>, End_Month <dbl>, End_Day <dbl>,
## #   Total_Deaths <dbl>, No_Injured <dbl>, No_Affected <dbl>, No_Homeless <dbl>,
## #   Total_Affected <dbl>, Recon_Costs <lgl>, Insured_Damages <dbl>,
## #   Total_Damages <dbl>, CPI <dbl>
```

***

# Accessing Columns and Rows
To access the columns, we could use `$` or `[]`; however, the output is slightly different. The example below will explain better:
```r
data["Year"] #Using the name of the column
```
Output:
```r
## # A tibble: 15,901 x 1
##     Year
##    <dbl>
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
```
***
```r
data[,"Year"] #Using "," to note that the first part is for rows and the second part is for columns. 
```
Output:
```r
## # A tibble: 15,901 x 1
##     Year
##    <dbl>
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
```
***
```r
data[1] #Using the index of the column
```
Output:
```r
## # A tibble: 15,901 x 1
##    ID_No        
##    <chr>        
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
```
Let's take a look at the type or `class` of the data that we have extracted:
```r
class(data["Year"])
```
Output:
```r
## [1] "tbl_df"     "tbl"        "data.frame"
```
It's a dataframe.

Whereas using `$` will return a vector of type numeric in this case (Since `Year` column is full of numeric values of years).
```r
head(data$Year)
```
Output:
```r
## [1] 1900 1900 1902 1902 1902 1903
```
***
```r
class(data$Year)
```
Output:
```r
## [1] "numeric"
```

So you might really need to pay attention to what you want to use.
You might call a function that applies only to vectors and not dataframes. Just know that `data["Year"]` and `data$Year` look similar but they are different.

To access rows, we can do:
```r
data[1,] #To access the first row
```
Output:
```r
## # A tibble: 1 x 43
##   ID_No          Year   Seq Disaster_Group Disaster_Subgroup Disaster_Type
##   <chr>         <dbl> <dbl> <chr>          <chr>             <chr>        
## 1 1900-9002-CPV  1900  9002 Natural        Climatological    Drought      
## # … with 37 more variables: Disaster_Subtype <chr>, Disaster_Subsubtype <chr>,
## #   Event_Name <chr>, Entry_Criteria <chr>, Country <chr>, ISO <chr>,
## #   Region <chr>, Continent <chr>, Location <chr>, Origin <chr>,
## #   Associated_Dis <chr>, Associated_Dis2 <chr>, OFDA_Response <chr>,
## #   Appeal <chr>, Declaration <chr>, Aid_Contribution <lgl>,
## #   Dis_Mag_Value <dbl>, Dis_Mag_Scale <chr>, Latitude <chr>, Longitude <chr>,
## #   Local_Time <chr>, River_Basin <chr>, Start_Year <dbl>, Start_Month <dbl>,
## #   Start_Day <dbl>, End_Year <dbl>, End_Month <dbl>, End_Day <dbl>,
## #   Total_Deaths <dbl>, No_Injured <dbl>, No_Affected <dbl>, No_Homeless <dbl>,
## #   Total_Affected <dbl>, Recon_Costs <lgl>, Insured_Damages <dbl>,
## #   Total_Damages <dbl>, CPI <dbl>
```

Yes, `0` is not the starting index in R. The first row is `1`.
You might find the `,` irritating at first but without that you are performing column access `data[1]`(Look at the example of accessing columns).
I personally think it's clearer to put a `,` showing where the row and column parts are.

**Boolean Indexing** is important. It opens so much capability to explore datasets. To retrieve a boolean vector of wehther the row belongs to Africa continent:
```r
head(data$Continent == "Africa")
```
Output:
```r
## [1]  TRUE FALSE FALSE FALSE FALSE FALSE
```
This returns a vector of `TRUE` and `FALSE`. Why is this important?
Well, this allows us to get the rows that have `Africa` as their `Continent`.
Put the boolean vector inside `[]` of the `data`.
```r
data[data$Continent == "Africa"]
```
Output:
```r
## Error: Must subset columns with a valid subscript vector.
## ℹ Logical subscripts must match the size of the indexed input.
## ✖ Input has size 43 but subscript `data$Continent == "Africa"` has size 15901.
```

Oops, looks like it didn't work! Yes, because we are using the boolean array to access columns and not rows. (Remember, the `,`!!!)
```r
data[data$Continent == "Africa", ]
```
Output:
```r
## # A tibble: 2,908 x 43
##    ID_No          Year   Seq Disaster_Group Disaster_Subgroup Disaster_Type    
##    <chr>         <dbl> <dbl> <chr>          <chr>             <chr>            
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
## # … with 2,898 more rows, and 37 more variables: Disaster_Subtype <chr>,
## #   Disaster_Subsubtype <chr>, Event_Name <chr>, Entry_Criteria <chr>,
## #   Country <chr>, ISO <chr>, Region <chr>, Continent <chr>, Location <chr>,
## #   Origin <chr>, Associated_Dis <chr>, Associated_Dis2 <chr>,
## #   OFDA_Response <chr>, Appeal <chr>, Declaration <chr>,
## #   Aid_Contribution <lgl>, Dis_Mag_Value <dbl>, Dis_Mag_Scale <chr>,
## #   Latitude <chr>, Longitude <chr>, Local_Time <chr>, River_Basin <chr>,
## #   Start_Year <dbl>, Start_Month <dbl>, Start_Day <dbl>, End_Year <dbl>,
## #   End_Month <dbl>, End_Day <dbl>, Total_Deaths <dbl>, No_Injured <dbl>,
## #   No_Affected <dbl>, No_Homeless <dbl>, Total_Affected <dbl>,
## #   Recon_Costs <lgl>, Insured_Damages <dbl>, Total_Damages <dbl>, CPI <dbl>
```
We have gained access to only the rows that have `Africa` as their `Continent`.

If you are like me who has to know every single thing going on before your fingers start to code and this concept of boolean indexing doesn't make sense to you, then let's think about it:
We accessed the column `Continent` and retrieved it as an array using `$`. Then we checked if the item in that array is `Africa`. This returns a boolean array of the length equivalent to:
```r
length(data$Continent == "Africa")
```
Output:
```r
## [1] 15901
```
which should be equivalent to the number of rows of the dataframe since it's originally extracted from there,
```r
length(rownames(data))
```
Output:
```r
## [1] 15901
```
Having the same length in the rows, we could use this boolean array as the criteria to filter out rows marked as `TRUE` in the dataframe.  

Some other useful things:
```r
subset(data, criteria ) #Subsetting from the dataframe using the criteria
x %in% y #Similar to in in python. Returns boolean datatype if x is in y
ifelse(test, if TRUE, if FALSE) #It is a way to run an if statement and return values accordingly in a single line
```

***

# Manipulating Data
Let's extract the important disasters that we want:
```r
fiveDis <- subset(data, Disaster_Type %in% c("Earthquake", "Drought", "Storm", "Flood", "Extreme temperature"))
```
What just happened? Well, first of all, `subset()` returns a subset of the dataframe using the criteria.
The criteria in our case is `Disaster_Type %in% c(,,,)`. 
`Disaster_Type` is a column in the dataframe.

**Wait**, shouldn't we use `data$Disaster_Type` to access that column?

Yes, but in `subset()`, we have already defined the data from which we are subsetting. Having defined what the original data is, we can just use the column name without the need to call the main data.

`c()` is R's way of making vectors. In our example, we have combined the five earthquakes' names in our vector using `c()`. Then we filtered out the disasters that belonged to these categories.

Now that we have the disasters that we need in `fiveDis`, let's get the continents:
```r
Africa <- fiveDis[fiveDis$Continent == "Africa", ]
```
It's a good practice to only keep the dataframe with least amount of columns that we need to use in our analysis:
```r
#Columns that we need:
colNeeded = c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")
Africa <- Africa[colNeeded]
```
Let's take a look at our data before moving on:
```r
head(Africa)
```
Output:
```r
## # A tibble: 6 x 6
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   <chr>     <dbl> <chr>                 <dbl>        <dbl>          <dbl>
## 1 Africa     1900 Drought                  NA        11000             NA
## 2 Africa     1910 Drought                  NA           NA             NA
## 3 Africa     1910 Drought                  NA           NA             NA
## 4 Africa     1910 Earthquake               NA           12             NA
## 5 Africa     1910 Drought                  NA           NA             NA
## 6 Africa     1910 Drought                  NA           NA             NA
```

It looks like we have so much NA values.
To solve this, we can first locate which rows have NA values and then replace the NA to something else in that row.
NA's are found in `Total_Damages`, `Total_Deaths`, and `Total_Affected`.

Use `is.na()` to get boolean array of whether the column has NA values.
```r
is.na(Africa$Total_Affected) #Returns boolean array. Use it in indexing
Africa[is.na(Africa$Total_Affected), ] #Now we have access to dataframe where Total_Affected column in this dataframe is full of NA values
Africa[is.na(Africa$Total_Affected), "Total_Affected"] #Now we have access to the column!
```
Now that we have access, what should we assign the values?
I personally thought that `median` values of disasters in that continent would be appropriate rather than `mean` values since disasters have outliers and mean values might heavily be impacted on the presence of outliers.

R comes from statistical background. It already has `mean()`, `median()`, `sd()` for standard deviation, `var()` for variance in samples, and so on.
Let's assign the median value of `Total_Affected` to each NA values in `Total_Affected` columns:
```r
Africa[is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa$Total_Affected)
```
It seems that there is no error, but our dataset does not look okay:
```r
head(Africa)
```
Output:
```r
## # A tibble: 6 x 6
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   <chr>     <dbl> <chr>                 <dbl>        <dbl>          <dbl>
## 1 Africa     1900 Drought                  NA        11000             NA
## 2 Africa     1910 Drought                  NA           NA             NA
## 3 Africa     1910 Drought                  NA           NA             NA
## 4 Africa     1910 Earthquake               NA           12             NA
## 5 Africa     1910 Drought                  NA           NA             NA
## 6 Africa     1910 Drought                  NA           NA             NA
```
That's because we had NA values in the main `Africa` dataset. What's the median value of `5, 6, 7, NA`? The answer is not clear. We have to remove the NA values while calculating for `median()`. We add the following flag `na.rm = TRUE`:
```r
Africa[is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa$Total_Affected, na.rm = TRUE)
```
Now the data looks fine. If you argue that each disaster group must have its own median value of `Total_Affected` values, I would agree. We could do something like:
```r
Africa[(Africa$Disaster_Type == "Storm") & is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa[Africa$Disaster_Type == "Storm", ]$Total_Affected, na.rm = TRUE)
```
The first part is finding the data where `Disaster_Type` is `Storm` and has NA values in the `Total_Affected` column, then assigning it with the median value of `Total_Affected` column where `Disaster_Type` is `Storm`.

Applying this to all the Disasters:
```r
Africa <- fiveDis[fiveDis$Continent == "Africa", c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")]

Africa[(Africa$Disaster_Type == "Drought") & is.na(Africa$Total_Damages), "Total_Damages"] <- median(Africa[Africa$Disaster_Type == "Drought", ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Earthquake") & is.na(Africa$Total_Damages), "Total_Damages"] <- median(Africa[Africa$Disaster_Type == "Earthquake", ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Extreme temperature") & is.na(Africa$Total_Damages), "Total_Damages"] <- median(Africa[Africa$Disaster_Type == "Extreme temperature", ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Flood") & is.na(Africa$Total_Damages), "Total_Damages"] <- median(Africa[Africa$Disaster_Type == "Flood", ]$Total_Damages, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Storm") & is.na(Africa$Total_Damages), "Total_Damages"] <- median(Africa[Africa$Disaster_Type == "Storm", ]$Total_Damages, na.rm = TRUE)

Africa[(Africa$Disaster_Type == "Drought") & is.na(Africa$Total_Deaths), "Total_Deaths"] <- median(Africa[Africa$Disaster_Type == "Drought", ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Earthquake") & is.na(Africa$Total_Deaths), "Total_Deaths"] <- median(Africa[Africa$Disaster_Type == "Earthquake", ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Extreme temperature") & is.na(Africa$Total_Deaths), "Total_Deaths"] <- median(Africa[Africa$Disaster_Type == "Extreme temperature", ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Flood") & is.na(Africa$Total_Deaths), "Total_Deaths"] <- median(Africa[Africa$Disaster_Type == "Flood", ]$Total_Deaths, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Storm") & is.na(Africa$Total_Deaths), "Total_Deaths"] <- median(Africa[Africa$Disaster_Type == "Storm", ]$Total_Deaths, na.rm = TRUE)

Africa[(Africa$Disaster_Type == "Drought") & is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa[Africa$Disaster_Type == "Drought", ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Earthquake") & is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa[Africa$Disaster_Type == "Earthquake", ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Extreme temperature") & is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa[Africa$Disaster_Type == "Extreme temperature", ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Flood") & is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa[Africa$Disaster_Type == "Flood", ]$Total_Affected, na.rm = TRUE)
Africa[(Africa$Disaster_Type == "Storm") & is.na(Africa$Total_Affected), "Total_Affected"] <- median(Africa[Africa$Disaster_Type == "Storm", ]$Total_Affected, na.rm = TRUE)
```

It looks messy! But for beginners, this is the best you could make out of base R, and I think it's important to understand the basics before moving on to higher level.

We have 5 Continents in total. Copying and pasting the above code for 5 continents is not so efficient, yet there is no other way for us now. At least that is what I did back when base R was the only thing I knew.

***

# **dplyr** in **tidyverse** package
For those experts in R who have long waited for me to start talking about `dplyr`, I can't believe you stayed with me so far (Although I'm pretty sure many would have left when they saw how inefficient I was). 

For all the beginners, would you believe if the above code could be simplified to:
```r
Africa <- fiveDis %>%
  filter(Continent == "Africa") %>%
  select(c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")) %>%
  group_by(Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()
```
*New things:*
To start with, `%>%` is called piping. Simply put, what it does is take whatever output we get before the `%>%` operator and put it as an input of what comes after the operator.

`filter()` is choosing the rows that meet certain criteria

`select()` is choosing the columns with the names given

`group_by()` is making partitions inside the dataframe, so that we could assess the groups individually

`mutate()` is modifying any existing column or making a new one

`ungroup()` is ungrouping groups made previously(We do this in case we get unexpected results that came out of assessing each groups individually when we wanted to assess the whole dataframe)

Breaking down the code:
```r
fiveDis %>%
  filter(Continent == "Africa") %>%
#Equivalent to fiveDis[fiveDis$Continent == "Africa", ]
```
Note that without the `%>%`, we would have needed to insert `fiveDis` inside the `filter()`:
```r
filter(fiveDis, Continent == "Africa")
```
Moving on, we use the newly acquired "filtered" dataframe and use it as an input for the next verb `select()`:
```r
select(c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")) %>%
```
We select the columns that we want, combine them as a list using `c()`, and then use it as the criteria to select the columns.

Next, we will group each `Disaster_Type` as a partition.
```r
group_by(Disaster_Type) %>%
```
The concept of grouping might seem esoteric at first, but it's useful.
`group_by` does not modify the dataframe itself and I mean by modifications such as sorting and subsetting:
```r
Africa <- fiveDis %>%
  filter(Continent == "Africa") %>%
  select(c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")) %>%
  group_by(Disaster_Type)
head(Africa)
```

Output:
```r
## # A tibble: 6 x 6
## # Groups:   Disaster_Type [2]
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   <chr>     <dbl> <chr>                 <dbl>        <dbl>          <dbl>
## 1 Africa     1900 Drought                  NA        11000             NA
## 2 Africa     1910 Drought                  NA           NA             NA
## 3 Africa     1910 Drought                  NA           NA             NA
## 4 Africa     1910 Earthquake               NA           12             NA
## 5 Africa     1910 Drought                  NA           NA             NA
## 6 Africa     1910 Drought                  NA           NA             NA
```
What `group_by()` does is adding a new information in the dataframe called `Groups` as shown in the code. We can see that `# Groups: Disaster_Type`. This will allow us to do statistical operations on each partition of `Disaster_Type`.

Let's see how `group_by()` helps in `mutate()`:
```r
mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
```
So, `mutate()` allows modification or addition of new columns. Looking at our first statement, we are modifying `Total_Damages` column. we test if `Total_Damages` have NA values. If it does, then assign to the NA value, the median of the `Total_Damages`. 
Is it assigning the median of all the values of `Total_Damages` in the `Africa` dataframe? No, here is where `group_by()` kicks in. Since we have already "partitioned" the dataframe into groups of disasters, when we say apply `median()` on this column, it will look for the individual groups/partitions to apply and assign values correspondingly. Again, `na.rm=TRUE` is to remove NA values since with the NA values included, we can't calculate `mean()` or `median()`.

If it's not NA value, then just assign it's own value. 

One more thing to note, we have said that `mutate()` allows the making a new column. You can see this in the last statement where we made a new column `Total_Casualty` which is equal to the sum of `Total_Affected` and `Total_Deaths`.

`ungroup()` is used to free the groups/partitions.

Our data looks like this after cleaning:
```r
Africa <- fiveDis %>%
  filter(Continent == "Africa") %>%
  select(c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")) %>%
  group_by(Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()
```
```r
Africa
```
Output:
```r
## # A tibble: 1,843 x 7
##    Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##    <chr>     <dbl> <chr>                 <dbl>        <dbl>          <dbl>
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
## # … with 1,833 more rows, and 1 more variable: Total_Casualty <dbl>
```

What if I want to have a dataframe that summarizes the influence of disasters per disaster type? Somethinng like, the average death by x disaster is y in z continent?

Have a look at the following code:
```r
#Average data for Africa Dataset
Africa_Avg <- Africa %>%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %>%
  group_by(Disaster_Type) %>%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n())
```
We know what `select()` and `group_by()` is. 

`summarize()` is a useful verb to apply a statistical function on a dataframe and get the result. It returns the summary of the dataframe. Inside `summarize()`, you'll need to include the new column names which you want to store the summarized data. I like to think of it as `mutate()`, making new columns with newly assigned data.

`n()` is a way to get the count of items in a group. As far as I know, `n()` only works inside `mutate()` and `summarize()` when we have defined the groups.  

Where's the `ungroup()`? `summarize()` takes off or "peels" off one group after it is performed. Since we only have one group `group_by(Disaster_Type)`, `summarize()` will leave us with no groups.

Let's see what happens:
```r
head(Africa_Avg)
```
Output:
```r
## # A tibble: 5 x 4
##   Disaster_Type       mean_Damage mean_Casualty count
##   <chr>                     <dbl>         <dbl> <int>
## 1 Drought                  49664.      1562370.   335
## 2 Earthquake              192743.        27539.    74
## 3 Extreme temperature      23904.       242663.    20
## 4 Flood                    17143.        80775.  1128
## 5 Storm                    39436.        83420.   286
```
That looks cleaner!

Let's apply this to all the continents:
```r
#Africa Data:
Africa <- fiveDis %>%
  filter(Continent == "Africa") %>%
  select(c("Continent", "Year", "Disaster_Type", "Total_Damages", "Total_Deaths", "Total_Affected")) %>%
  group_by(Continent, Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()

#Average data for Africa Dataset
Africa_Avg <- Africa %>%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %>%
  group_by(Continent, Disaster_Type) %>%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %>%
  ungroup()

#America Data
America <- fiveDis %>%
  filter(Continent == "Americas") %>%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %>%
  group_by(Continent, Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()

America_Avg <- America %>%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %>%
  group_by(Continent, Disaster_Type) %>%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %>%
  ungroup()

#Asia Data
Asia <- fiveDis %>%
  filter(Continent == "Asia") %>%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %>%
  group_by(Continent, Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()

Asia_Avg <- Asia %>%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %>%
  group_by(Continent, Disaster_Type) %>%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %>%
  ungroup()

#Europe Data
Europe <- fiveDis %>%
  filter(Continent == "Europe") %>%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %>%
  group_by(Continent, Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()

Europe_Avg <- Europe %>%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %>%
  group_by(Continent, Disaster_Type) %>%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %>%
  ungroup()

#Oceania Data
Oceania <- fiveDis %>%
  filter(Continent == "Oceania") %>%
  select(c(Continent, Year, Disaster_Type, Total_Damages, Total_Deaths, Total_Affected)) %>%
  group_by(Continent, Disaster_Type) %>%
  mutate(Total_Damages = ifelse(is.na(Total_Damages), median(Total_Damages, na.rm = TRUE), Total_Damages),
         Total_Deaths = ifelse(is.na(Total_Deaths), median(Total_Deaths, na.rm = TRUE), Total_Deaths),
         Total_Affected = ifelse(is.na(Total_Affected), median(Total_Affected, na.rm = TRUE), Total_Affected),
         Total_Casualty = Total_Affected + Total_Deaths) %>%
  ungroup()

Oceania_Avg <- Oceania %>%
  select(Continent, Disaster_Type, Total_Damages, Total_Casualty) %>%
  group_by(Continent, Disaster_Type) %>%
  summarize(mean_Damage = mean(Total_Damages), mean_Casualty = mean(Total_Casualty), count = n()) %>%
  ungroup()
```

Before we move on to plotting, I would like to cover one more thing.
You might want to combine dataframes row-wise or column-wise according to what you want to do with your data.
`rbind()` and `cbind()` are useful to combine rows and columns.

If I want to make one dataframe out of the continents with the NA values removed:
```r
fiveDis_narm <- rbind(Africa, America, Asia, Europe, Oceania)
```
I could run the above code to do so.

Moving further, I could include how many counts are there for each `Continent`-`Disaster_Type` pairs using `n()`:
```r
fiveDis_narm <- rbind(Africa, America, Asia, Europe, Oceania) %>%
  group_by(Continent, Disaster_Type) %>%
  mutate(count = n()) %>%
  ungroup()
```
```r
head(fiveDis_narm)
```
Output:
```r
## # A tibble: 6 x 8
##   Continent  Year Disaster_Type Total_Damages Total_Deaths Total_Affected
##   <chr>     <dbl> <chr>                 <dbl>        <dbl>          <dbl>
## 1 Africa     1900 Drought               34000        11000         838000
## 2 Africa     1910 Drought               34000          237         838000
## 3 Africa     1910 Drought               34000          237         838000
## 4 Africa     1910 Earthquake            28000           12           2000
## 5 Africa     1910 Drought               34000          237         838000
## 6 Africa     1910 Drought               34000          237         838000
## # … with 2 more variables: Total_Casualty <dbl>, count <int>
```

I could do the same thing for the Average values of disasters per continent:
```r
continent_Avg <- rbind(Africa_Avg, America_Avg, Asia_Avg, Europe_Avg, Oceania_Avg)
```

Alright, our dataframes are ready. Let's plot!

***

# Plots using `ggplot`

Let's make a quick dataframe that counts the occurrences of disasters in each year in the dataset:
```r
count_Year <- data %>%
  group_by(Year) %>%
  summarize(count = n()) %>%
  ungroup()
```
Simple and easy. It looks like this:
```r
head(count_Year)
```
Output:
```r
## # A tibble: 6 x 2
##    Year count
##   <dbl> <int>
## 1  1900     7
## 2  1901     2
## 3  1902    10
## 4  1903    12
## 5  1904     4
## 6  1905     8
```
Using this dataframe, we can plot the counts of disasters for each year.
`ggplot()` takes `data` and `aes()` to start drawing the "skeleton" of the plot. Consider this as the frame in which we will start drawing our plots by adding `layers` to it.

```r
ggplot(data = count_Year, aes(x = Year, y = count))
```
Output:
![ggplot](/assets/img/R-Basics-plot-1.png)

You might wonder why it did not show any drawing. That's because we mapped the variables/columns to `x` and `y` but did not specify what to draw with it. Am I telling it to draw a linechart? a scatterplot? a barplot?

We specify this by adding layers.
`geom_point()` adds points to the plot making it a scatterplot.

```r
ggplot(data = count_Year, aes(x = Year, y = count)) + 
  geom_point()
```
Output:
![ggplot](/assets/img/R-Basics-plot-2.png)


Don't forget to put `+` everytime you want to "add" a layer.

You might want to go and check for the documentation to see lots and lots of layers you could add to make your plots more insightful.
 

For example, you could do something like this:

```r
ggplot(data = count_Year, aes(x = Year, y = count)) +
  geom_point() + #Scatterplot
  geom_text(aes(label = count), check_overlap = TRUE, vjust = -1) + #Labeling the counts
  scale_x_continuous("Year", breaks = c(1900, 1920, 1940, 1960, 1980, 1988, 2000, 2020), limits = c(1900, 2020))+ #Scaling the x-axis and giving it a label
  scale_y_continuous("Number of Disasters", breaks = c(0, 20, 50, 100, 200, 300, 400, 500), limits = c(0, 600)) +
  geom_vline(xintercept = 1988, linetype = "dotted", color = "red", size = 1) + #Adding a vertical line to the plot
  labs(title = "Total Number of Disasters Occurring in Each Year",
        subtitle = "Disasters start to occur more frequently after the year 1988, the year that marks the beginning of climate change")+ #Labelling title and subtitle
  theme_bw() + #Adding the overall theme to the plot
  theme(plot.title = element_text(face = "bold", size = 17), 
        plot.subtitle = element_text(face = "italic", size = 10)) + #Modifying the title and subtitle
  stat_smooth(se = FALSE, col = "gray") #Adding smoothing line
```

Output:
![ggplot](/assets/img/R-Basics-plot-3.png)

Looks like the number of disasters are increasing over time. 

***

Using the dataframes that we made previously, we could plot the following graphs:

```r
ggplot(continent_Avg, aes(color = Disaster_Type)) +
      geom_point(aes(y = mean_Casualty / 1000, x = mean_Damage/ 1000, shape = Disaster_Type), alpha = 0.5, size = 5) +
      facet_wrap(~Continent, scale="free") +
      scale_shape_manual(values = c(15, 16, 17, 18, 20)) +
      scale_y_continuous("Average number of casualties in (,000) person", trans = "log10") + 
      scale_x_continuous("Average damage in (,000,000) dollars", trans = "log10") +
      scale_color_manual(values = c("#cc9900", "#663300", "#ff0000", "#0066dd", "#999999")) +
      labs(title = "How critical are disasters in your continent?") +
      theme(plot.title = element_text(face = "bold", size = 17))
```

Output:
![ggplot](/assets/img/R-Basics-plot-4.png)

***

```r
ggplot(data = fiveDis, aes(x = Year, color = Disaster_Type)) +
    geom_line(stat = "count", alpha = 0.6, size = 1)+
    scale_x_continuous("Year of Occurrence", breaks = c(1900, 1980, 1988, 2020), limits = c(1900, 2020)) +
    scale_y_continuous("Number of Disasters", breaks = c(0, 10, 30, 50, 100, 150, 200)) + 
    geom_vline(xintercept = 1988, linetype = "dotted", color = "red", size = 1) +
    theme_bw() +
    labs(title = "Five Big Disasters in the World",
          subtitle = "marking the year 1988, the year which faced the beginning of climate change") +
    theme(plot.title = element_text(face = "bold", size = 17),
          plot.subtitle = element_text(face = "italic", size = 10))
```

Output:
![ggplot](/assets/img/R-Basics-plot-5.png)

***

# Discovering R

***

## - `Shiny`

You might want your plots to be interactive, in a sense that you want to display a different plot whenever the audience selects something as an input. `Shiny` package is the one you want to use. 

```r
library(shiny)
```

Sadly, it looks like Kaggle's notebook script does not allow the usage of interactive shiny `renderplot()` function. It keeps throwing errors.

However, feel free to check out my implemetation of the Disasters data using Shiny [here](https://ihazun01.shinyapps.io/Deploy/).

***

## - `wordcloud`

`wordcloud` is a package that allows us to perform data analysis based on texts and words.

We could know what the most frequent disaster was in the year 2020:

```r
library(wordcloud)

hotDis2020 <- data %>%
  filter(Year == 2020) %>%
  group_by(Disaster_Type) %>%
  summarize(count = n())
wordcloud(word = hotDis2020$Disaster_Type, freq = hotDis2020$count, color="Red")
```

Output:
![ggplot](/assets/img/R-Basics-plot-6.png)

***

# Conclusion

***
R opens up so many capabilities to analyze data and draw plots. If you are interested in any kind of plots, go look for it in the R documentation. I hope this tutorial and analysis stimulates you to explore and discover more of R!

***

## Acknowledgements
This dataset has been acquired from [EM-DAT](https://www.emdat.be/) database. I hereby acknowledge the source of the dataset is EM-DAT. **_`EM-DAT: The Emergency Events Database - Université catholique de Louvain (UCL) - CRED, D. Guha-Sapir - www.emdat.be, Brussels, Belgium.`_**

***


**_`“Everyone has a plan until they get punched in the mouth.”`_**
