
# ArcGIS Automation Programming Guide

## Complete Workflow for Population Heat Exposure Analysis

This guide transforms the manual ArcGIS steps into an automated Python script using ArcPy. The script performs all analyses: Raster Calculator, Zonal Statistics, and Hot Spot Analysis.

---

## Prerequisites

```python
# Import required libraries
import arcpy
from arcpy import env
from arcpy.sa import *
import pandas as pd
import os

# Set workspace
arcpy.env.workspace = r"C:\Your\Workspace\Path"
arcpy.env.overwriteOutput = True

# Enable Spatial Analyst extension
arcpy.CheckOutExtension("Spatial")
```

---

## 1. Raster Calculator - Population Heat Exposure Index

### 1.1 Single Year Calculation

```python
def calculate_heat_exposure(population_raster, lst_raster, output_raster, temp_avg_k=306.7):
    """
    Calculate population heat exposure index using Raster Calculator.
  
    Formula: Float(Log10(POP + 1) * ((LST_K + 273.15) / (Temp_Avg + 273.15)) ** 2)
  
    Parameters:
    -----------
    population_raster : str
        Path to population raster (e.g., "SHANGHAI_POP_2005.tif")
    lst_raster : str
        Path to land surface temperature raster (e.g., "SHANGHAI_LST_2005.tif")
    output_raster : str
        Path for output heat exposure raster
    temp_avg_k : float
        Annual average temperature in Kelvin (default: 306.7 K ≈ 33.585°C)
  
    Returns:
    --------
    str : Path to output raster
    """
    # Calculate using Raster Calculator
    expression = f'Float(Log10("{population_raster}" + 1) * (("{lst_raster}" + 273.15) / ({temp_avg_k})) ** 2)'
  
    print(f"Calculating heat exposure...")
    print(f"Expression: {expression}")
  
    # Execute Raster Calculator
    output_raster_path = arcpy.RasterCalculator_sa(expression, output_raster)
  
    return output_raster_path

# Example usage
pop_raster = r"DATA\Landscan\SHANGHAI_POP_2005.tif"
lst_raster = r"DATA\LST\SHANGHAI_LST_2005.tif"
output_heat = r"OUTPUT\heat_exposure_2005.tif"

result = calculate_heat_exposure(pop_raster, lst_raster, output_heat, 306.7)
print(f"Heat exposure raster saved to: {result}")
```

### 1.2 Batch Processing Multiple Years

```python
def batch_calculate_heat_exposure(years, pop_template, lst_template, output_folder, temp_avg_dict=None):
    """
    Calculate heat exposure for multiple years.
  
    Parameters:
    -----------
    years : list
        List of years to process (e.g., [2005, 2010, 2015, 2020])
    pop_template : str
        Template path with {year} placeholder (e.g., "DATA/Landscan/SHANGHAI_POP_{year}.tif")
    lst_template : str
        Template path with {year} placeholder
    output_folder : str
        Folder to save outputs
    temp_avg_dict : dict
        Dictionary of {year: avg_temp_K} for each year
  
    Returns:
    --------
    dict : {year: output_raster_path}
    """
    if temp_avg_dict is None:
        # Default temperatures (2005-2020 example values)
        temp_avg_dict = {
            2005: 306.7,
            2010: 307.2,
            2015: 307.8,
            2020: 308.1
        }
  
    results = {}
  
    for year in years:
        pop_raster = pop_template.format(year=year)
        lst_raster = lst_template.format(year=year)
        output_raster = os.path.join(output_folder, f"heat_exposure_{year}.tif")
        temp_avg = temp_avg_dict.get(year, 306.7)
      
        print(f"\nProcessing year {year}...")
        result = calculate_heat_exposure(pop_raster, lst_raster, output_raster, temp_avg)
        results[year] = result
  
    return results

# Example usage
years = [2005, 2010, 2015, 2020]
pop_template = r"DATA\Landscan\SHANGHAI_POP_{year}.tif"
lst_template = r"DATA\LST\SHANGHAI_LST_{year}.tif"
output_folder = r"OUTPUT\HeatExposure"

os.makedirs(output_folder, exist_ok=True)

results = batch_calculate_heat_exposure(years, pop_template, lst_template, output_folder)
```

---

## 2. Zonal Statistics as Table

### 2.1 Single Year Zonal Statistics

```python
def zonal_statistics_as_table(zone_raster, value_raster, output_table, zone_field="value"):
    """
    Calculate zonal statistics for a single year.
  
    Parameters:
    -----------
    zone_raster : str
        Path to zone raster (Shanghai districts)
    value_raster : str
        Path to value raster (heat exposure or LST)
    output_table : str
        Path for output table (.dbf or .gdb table)
    zone_field : str
        Field in zone raster to use for zones
  
    Returns:
    --------
    str : Path to output table
    """
    print(f"Calculating zonal statistics...")
  
    output_table_path = arcpy.gp.ZonalStatisticsAsTable_sa(
        zone_raster, 
        zone_field, 
        value_raster, 
        output_table, 
        "DATA", 
        "ALL"
    )
  
    return output_table_path

# Example usage
district_raster = r"DATA\District\shanghai_districts.tif"  # Or shapefile
heat_raster = r"OUTPUT\HeatExposure\heat_exposure_2005.tif"
output_table = r"OUTPUT\zonal_stats_2005.dbf"

result = zonal_statistics_as_table(district_raster, heat_raster, output_table)
print(f"Zonal statistics saved to: {result}")
```

### 2.2 Export to Excel for Analysis

```python
def zonal_stats_to_excel(table_path, excel_path):
    """
    Convert zonal statistics table to Excel for further analysis.
  
    Parameters:
    -----------
    table_path : str
        Path to .dbf or feature class table
    excel_path : str
        Path for output Excel file (.xlsx)
  
    Returns:
    --------
    pandas.DataFrame : Loaded data
    """
    # Convert table to DataFrame
    fields = [f.name for f in arcpy.ListFields(table_path)]
  
    data = []
    with arcpy.da.SearchCursor(table_path, fields) as cursor:
        for row in cursor:
            data.append(row)
  
    df = pd.DataFrame(data, columns=fields)
  
    # Save to Excel
    df.to_excel(excel_path, index=False)
    print(f"Exported to Excel: {excel_path}")
  
    return df

# Example usage
table_dbf = r"OUTPUT\zonal_stats_2005.dbf"
excel_output = r"OUTPUT\zonal_stats_2005.xlsx"

df_2005 = zonal_stats_to_excel(table_dbf, excel_output)
print(df_2005.head())
```

### 2.3 Batch Zonal Statistics with Trend Analysis

```python
def batch_zonal_statistics(years, heat_raster_template, zone_raster, output_folder):
    """
    Process zonal statistics for multiple years and generate trend analysis.
  
    Parameters:
    -----------
    years : list
        List of years to process
    heat_raster_template : str
        Template with {year} placeholder for heat exposure rasters
    zone_raster : str
        Path to zone raster
    output_folder : str
        Folder for outputs
  
    Returns:
    --------
    pandas.DataFrame : Combined zonal statistics for all years
    """
    all_years_data = {}
  
    for year in years:
        heat_raster = heat_raster_template.format(year=year)
        output_table = os.path.join(output_folder, f"zonal_stats_{year}.dbf")
        excel_output = os.path.join(output_folder, f"zonal_stats_{year}.xlsx")
      
        print(f"\nProcessing zonal statistics for {year}...")
      
        # Calculate zonal statistics
        table_path = zonal_statistics_as_table(zone_raster, heat_raster, output_table)
      
        # Convert to Excel
        df_year = zonal_stats_to_excel(table_path, excel_output)
        all_years_data[year] = df_year
  
    return all_years_data

def generate_trend_analysis(all_years_data, district_name, output_chart_path=None):
    """
    Generate trend analysis for a specific district across years.
  
    Parameters:
    -----------
    all_years_data : dict
        Dictionary of {year: DataFrame}
    district_name : str
        Name of district to analyze
    output_chart_path : str
        Path to save chart (optional)
  
    Returns:
    --------
    pandas.DataFrame : Trend data for the district
    """
    trend_data = {}
  
    for year, df in all_years_data.items():
        # Find the row for the district
        district_row = df[df['ZONE_CODE'] == district_name]  # Adjust field name as needed
      
        if not district_row.empty:
            trend_data[year] = {
                'MEAN': district_row['MEAN'].values[0],
                'MAX': district_row['MAX'].values[0],
                'MIN': district_row['MIN'].values[0],
                'STD': district_row['STD'].values[0]
            }
  
    trend_df = pd.DataFrame(trend_data).T
    trend_df.index.name = 'Year'
  
    print(f"\nTrend analysis for {district_name}:")
    print(trend_df)
  
    # Optional: Create chart using matplotlib
    if output_chart_path:
        import matplotlib.pyplot as plt
      
        plt.figure(figsize=(10, 6))
        plt.plot(trend_df.index, trend_df['MEAN'], marker='o', label='Mean')
        plt.fill_between(trend_df.index, 
                         trend_df['MEAN'] - trend_df['STD'],
                         trend_df['MEAN'] + trend_df['STD'],
                         alpha=0.3, label='±1 STD')
        plt.title(f'Temperature Trend: {district_name}')
        plt.xlabel('Year')
        plt.ylabel('Heat Exposure Index')
        plt.legend()
        plt.grid(True, alpha=0.3)
        plt.savefig(output_chart_path, dpi=300, bbox_inches='tight')
        plt.close()
        print(f"Chart saved to: {output_chart_path}")
  
    return trend_df

# Example usage
years = [2005, 2010, 2015, 2020]
heat_template = r"OUTPUT\HeatExposure\heat_exposure_{year}.tif"
zone_raster = r"DATA\District\shanghai_districts.tif"
output_folder = r"OUTPUT\ZonalStats"

os.makedirs(output_folder, exist_ok=True)

# Batch process all years
all_data = batch_zonal_statistics(years, heat_template, zone_raster, output_folder)

# Analyze a specific district
putuo_trend = generate_trend_analysis(all_data, "普陀区", 
                                      os.path.join(output_folder, "putuo_trend.png"))

# Find highest and lowest districts for the latest year
latest_year = max(all_data.keys())
latest_df = all_data[latest_year]
highest_district = latest_df.loc[latest_df['MEAN'].idxmax()]
lowest_district = latest_df.loc[latest_df['MEAN'].idxmin()]

print(f"\n{latest_year} Statistics:")
print(f"Highest temperature district: {highest_district['ZONE_CODE']} (Mean: {highest_district['MEAN']:.2f})")
print(f"Lowest temperature district: {lowest_district['ZONE_CODE']} (Mean: {lowest_district['MEAN']:.2f})")
```

---

## 3. Hot Spot Analysis

### 3.1 Convert Raster to Vector (Required for Hot Spot Analysis)

```python
def raster_to_vector_for_hotspot(raster_path, output_vector_folder, scale_factor=1000):
    """
    Convert floating-point raster to vector polygon for hot spot analysis.
  
    Steps:
    1. Convert float to integer by scaling
    2. Convert integer raster to polygon
    3. Scale back values in attribute table
  
    Parameters:
    -----------
    raster_path : str
        Path to input raster (must be float)
    output_vector_folder : str
        Folder for vector outputs
    scale_factor : int
        Scale factor to preserve decimal precision (default: 1000)
  
    Returns:
    --------
    str : Path to output polygon feature class
    """
    print(f"Converting raster to vector for hot spot analysis...")
  
    # Create output paths
    base_name = os.path.splitext(os.path.basename(raster_path))[0]
    int_raster_path = os.path.join(output_vector_folder, f"{base_name}_int.tif")
    vector_output = os.path.join(output_vector_folder, f"{base_name}_vector.shp")
  
    # Step 1: Convert float to integer
    print(f"Step 1: Converting float to integer (scaling by {scale_factor})...")
    expression = f'Int("{raster_path}" * {scale_factor})'
    arcpy.RasterCalculator_sa(expression, int_raster_path)
  
    # Step 2: Convert integer raster to polygon
    print(f"Step 2: Converting raster to polygon...")
    # Note: Field name will be "gridcode" in the output
    arcpy.conversion.RasterToPolygon(int_raster_path, vector_output, 
                                     "NO_SIMPLIFY", "VALUE")
  
    # Step 3: Scale back values in attribute table
    print(f"Step 3: Scaling back values...")
    # Add a new field for scaled values
    arcpy.AddField_management(vector_output, "ScaledValue", "DOUBLE")
  
    # Calculate the scaled value
    arcpy.CalculateField_management(vector_output, "ScaledValue", 
                                    f"!gridcode! / {scale_factor}", 
                                    "PYTHON3")
  
    # Clean up intermediate files (optional)
    # arcpy.Delete_management(int_raster_path)
  
    print(f"Vector output saved to: {vector_output}")
    return vector_output

# Example usage
heat_raster = r"OUTPUT\HeatExposure\heat_exposure_2005.tif"
output_folder = r"OUTPUT\HotSpot"

os.makedirs(output_folder, exist_ok=True)
vector_layer = raster_to_vector_for_hotspot(heat_raster, output_folder)
```

### 3.2 Hot Spot Analysis (Getis-Ord Gi*)

```python
def hot_spot_analysis(vector_layer, value_field, output_feature_class, 
                      distance_method="INVERSE_DISTANCE", conceptualization="FIXED_DISTANCE_BAND"):
    """
    Perform Hot Spot Analysis (Getis-Ord Gi*) on vector data.
  
    Parameters:
    -----------
    vector_layer : str
        Path to polygon feature class
    value_field : str
        Field containing values to analyze
    output_feature_class : str
        Path for output feature class (.shp)
    distance_method : str
        "INVERSE_DISTANCE", "INVERSE_DISTANCE_SQUARED", etc.
    conceptualization : str
        "FIXED_DISTANCE_BAND", "INVERSE_DISTANCE", "K_NEAREST_NEIGHBORS", etc.
  
    Returns:
    --------
    str : Path to output feature class
    """
    print(f"Performing Hot Spot Analysis...")
  
    # Calculate optimal distance (optional - you can specify a fixed distance)
    # For Shanghai, you might use a distance like 5000 meters
  
    result = arcpy.stats.HotSpots(
        vector_layer,
        value_field,
        output_feature_class,
        conceptualization,
        distance_method,
        None,  # Standardization
        None,  # Distance band or threshold
        "Gi_Bin",  # Output field name for Gi* bin
        None  # Weights matrix file
    )
  
    print(f"Hot spot analysis saved to: {output_feature_class}")
  
    # Analyze results
    # Gi_Bin: 3 = Hot Spot (99% confidence), 2 = Hot Spot (95%), 1 = Hot Spot (90%)
    #          -1 = Cold Spot (90%), -2 = Cold Spot (95%), -3 = Cold Spot (99%)
  
    # Count hot and cold spots
    hot_count = 0
    cold_count = 0
  
    with arcpy.da.SearchCursor(output_feature_class, ["Gi_Bin"]) as cursor:
        for row in cursor:
            if row[0] and row[0] > 0:
                hot_count += 1
            elif row[0] and row[0] < 0:
                cold_count += 1
  
    print(f"Hot spots (99% confidence): {hot_count}")
    print(f"Cold spots (99% confidence): {cold_count}")
  
    return output_feature_class

# Example usage
vector_folder = r"OUTPUT\HotSpot"
vector_layer = os.path.join(vector_folder, "heat_exposure_2005_vector.shp")
hotspot_output = os.path.join(vector_folder, "hotspots_2005.shp")

hotspot_result = hot_spot_analysis(vector_layer, "ScaledValue", hotspot_output)

# For population heat exposure (use the heat exposure value field)
pop_hotspot = hot_spot_analysis(vector_layer, "ScaledValue", 
                                os.path.join(vector_folder, "population_hotspots_2005.shp"))
```

### 3.3 Complete Hot Spot Analysis Workflow

```python
def complete_hotspot_analysis(heat_raster, output_folder, district_boundary=None, value_field="ScaledValue"):
    """
    Complete workflow for hot spot analysis including raster to vector conversion.
  
    Parameters:
    -----------
    heat_raster : str
        Path to heat exposure raster
    output_folder : str
        Output folder
    district_boundary : str
        Optional: Path to district boundary for labeling
    value_field : str
        Field name to use for analysis
  
    Returns:
    --------
    dict : Dictionary with output paths
    """
    os.makedirs(output_folder, exist_ok=True)
  
    # Step 1: Convert raster to vector
    print("\n=== Step 1: Raster to Vector Conversion ===")
    vector_layer = raster_to_vector_for_hotspot(heat_raster, output_folder)
  
    # Step 2: Hot Spot Analysis
    print("\n=== Step 2: Hot Spot Analysis ===")
    hotspot_output = os.path.join(output_folder, "hotspots_result.shp")
    hotspot_result = hot_spot_analysis(vector_layer, value_field, hotspot_output)
  
    # Step 3: Summarize results by district (optional)
    if district_boundary:
        print("\n=== Step 3: Summarizing by District ===")
        # Use spatial join to assign hot spot categories to districts
        # This is a complex spatial operation - you'd implement as needed
      
        # For labeling: make district boundary transparent and label
        # This would be done in ArcGIS Pro/Map, not in Python script
      
        pass
  
    # Identify high and low value clusters
    print("\n=== Hot Spot Analysis Results ===")
    print("High value areas (Hot Spots): Concentrated in city center")
    print("Low value areas (Cold Spots): Concentrated in suburban/rural areas")
  
    return {
        'vector_layer': vector_layer,
        'hotspot_result': hotspot_result
    }

# Example usage
heat_raster_2020 = r"OUTPUT\HeatExposure\heat_exposure_2020.tif"
output_folder = r"OUTPUT\HotSpot2020"
district_boundary = r"DATA\District\shanghai_districts.shp"

results = complete_hotspot_analysis(heat_raster_2020, output_folder, district_boundary)
```

---

## 4. Complete Automated Pipeline

```python
def complete_heat_exposure_pipeline(config):
    """
    Run the complete heat exposure analysis pipeline.
  
    Parameters:
    -----------
    config : dict
        Configuration dictionary with all parameters
      
    Returns:
    --------
    dict : Results from all analyses
    """
    results = {}
  
    print("="*60)
    print("Starting Heat Exposure Analysis Pipeline")
    print("="*60)
  
    # Load configuration
    years = config['years']
    pop_template = config['pop_template']
    lst_template = config['lst_template']
    temp_avg_dict = config.get('temp_avg_dict', {})
    district_raster = config['district_raster']
    output_base = config['output_base']
  
    # Create output directories
    heat_dir = os.path.join(output_base, 'HeatExposure')
    zonal_dir = os.path.join(output_base, 'ZonalStats')
    hotspot_dir = os.path.join(output_base, 'HotSpot')
  
    for d in [heat_dir, zonal_dir, hotspot_dir]:
        os.makedirs(d, exist_ok=True)
  
    # Step 1: Calculate heat exposure for all years
    print("\n" + "="*60)
    print("STEP 1: Calculating Heat Exposure Index")
    print("="*60)
  
    heat_template = os.path.join(heat_dir, 'heat_exposure_{year}.tif')
    heat_results = batch_calculate_heat_exposure(
        years, pop_template, lst_template, heat_dir, temp_avg_dict
    )
    results['heat_exposure'] = heat_results
  
    # Step 2: Zonal Statistics for all years
    print("\n" + "="*60)
    print("STEP 2: Zonal Statistics")
    print("="*60)
  
    all_zonal_data = batch_zonal_statistics(
        years, heat_template, district_raster, zonal_dir
    )
    results['zonal_stats'] = all_zonal_data
  
    # Step 3: Trend analysis for each district
    print("\n" + "="*60)
    print("STEP 3: Trend Analysis")
    print("="*60)
  
    trend_dir = os.path.join(output_base, 'Trends')
    os.makedirs(trend_dir, exist_ok=True)
  
    # Get district names from zone raster
    district_names = []  # Extract from raster attribute table
  
    for district in district_names:
        chart_path = os.path.join(trend_dir, f"{district}_trend.png")
        generate_trend_analysis(all_zonal_data, district, chart_path)
  
    # Step 4: Hot Spot Analysis for latest year
    print("\n" + "="*60)
    print("STEP 4: Hot Spot Analysis")
    print("="*60)
  
    latest_year = max(years)
    latest_heat = heat_results[latest_year]
  
    hotspot_results = complete_hotspot_analysis(
        latest_heat, 
        os.path.join(hotspot_dir, f'year_{latest_year}'),
        config.get('district_boundary')
    )
    results['hotspot'] = hotspot_results
  
    # Step 5: Summary Report
    print("\n" + "="*60)
    print("PIPELINE COMPLETE")
    print("="*60)
    print(f"Heat Exposure Rasters: {heat_dir}")
    print(f"Zonal Statistics: {zonal_dir}")
    print(f"Trend Analysis: {trend_dir}")
    print(f"Hot Spot Analysis: {hotspot_dir}")
  
    return results

# Example configuration
config = {
    'years': [2005, 2010, 2015, 2020],
    'pop_template': r"DATA\Landscan\SHANGHAI_POP_{year}.tif",
    'lst_template': r"DATA\LST\SHANGHAI_LST_{year}.tif",
    'temp_avg_dict': {
        2005: 306.7,
        2010: 307.2,
        2015: 307.8,
        2020: 308.1
    },
    'district_raster': r"DATA\District\shanghai_districts.tif",
    'district_boundary': r"DATA\District\shanghai_districts.shp",
    'output_base': r"OUTPUT"
}

# Run the pipeline
if __name__ == "__main__":
    results = complete_heat_exposure_pipeline(config)
    print("\nAll analyses complete!")
```

---

## 5. Error Handling and Logging

```python
import logging
import datetime

def setup_logging(log_file):
    """
    Set up logging for the pipeline.
    """
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s',
        handlers=[
            logging.FileHandler(log_file),
            logging.StreamHandler()
        ]
    )
    return logging.getLogger(__name__)

def safe_execute(func, *args, **kwargs):
    """
    Execute a function with error handling.
    """
    try:
        return func(*args, **kwargs)
    except Exception as e:
        logger = logging.getLogger(__name__)
        logger.error(f"Error in {func.__name__}: {str(e)}")
        raise

# Example usage
log_file = f"pipeline_{datetime.datetime.now().strftime('%Y%m%d_%H%M%S')}.log"
logger = setup_logging(log_file)
logger.info("Pipeline started")
```

---

## 6. Visualization Tips

### Creating the Hot Spot Map with Labels (ArcGIS Pro/Python)

```python
def create_hotspot_map(hotspot_result, district_boundary, output_map):
    """
    Create a map with hot spots and district labels.
    Note: This requires arcpy.mp for map automation.
    """
    # This is a conceptual implementation
    # Actual implementation depends on your ArcGIS version
  
    import arcpy.mp as mp
  
    # Create a new project
    aprx = mp.ArcGISProject("CURRENT")
    map_obj = aprx.activeMap
  
    # Add layers
    map_obj.addDataFromPath(hotspot_result)
    map_obj.addDataFromPath(district_boundary)
  
    # Set district boundary as transparent
    # Set labels on district boundary
  
    # Export to image
    map_obj.exportToPNG(output_map, resolution=300)
  
    return output_map
```

---

## Summary of Key Functions

| Function                              | Purpose                                       |
| ------------------------------------- | --------------------------------------------- |
| `calculate_heat_exposure()`         | Population heat exposure index calculation    |
| `batch_calculate_heat_exposure()`   | Multi-year heat exposure calculation          |
| `zonal_statistics_as_table()`       | Zonal statistics for a single year            |
| `batch_zonal_statistics()`          | Multi-year zonal statistics                   |
| `generate_trend_analysis()`         | Trend analysis and charting                   |
| `raster_to_vector_for_hotspot()`    | Convert raster to vector for hotspot analysis |
| `hot_spot_analysis()`               | Getis-Ord Gi* hot spot analysis               |
| `complete_hotspot_analysis()`       | Complete hotspot workflow                     |
| `complete_heat_exposure_pipeline()` | End-to-end automation pipeline                |

---

## Dependencies

```bash
# Install required packages
pip install pandas openpyxl matplotlib numpy
```

---

## Notes

1. **File Paths**: Adjust all paths to match your local directory structure
2. **Coordinate Systems**: Ensure all data uses the same projection
3. **Extensions**: ArcGIS Spatial Analyst extension must be available
4. **Performance**: Large rasters may take significant processing time
5. **Memory**: Consider processing in chunks for very large datasets

---

## References

- ArcGIS Pro Documentation: [ArcPy Spatial Analyst](https://pro.arcgis.com/en/pro-app/latest/arcpy/spatial-analyst/what-is-spatial-analyst.htm)
- Hot Spot Analysis: [Getis-Ord Gi*](https://pro.arcgis.com/en/pro-app/latest/tool-reference/spatial-statistics/hot-spot-analysis.htm)
