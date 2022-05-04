USE [master]
GO
CREATE DATABASE [provider]
GO
USE [provider]
GO
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[ProvMaster](
	[ProvMasterKey] [int] IDENTITY(1,1) NOT NULL,
	[MedicaidProvID] [int] NOT NULL,
	[NPI] [int] NOT NULL,
	[ProvFacName] [varchar](255) NOT NULL,
	[MedicaidType] [varchar](40) NOT NULL,
	[ProvType] [varchar](255) NULL,
	[ProvSpecialty] [varchar](255) NULL,
	[ServiceAddress] [varchar](100) NULL,
	[City] [varchar](60) NULL,
	[State] [varchar](02) NULL,
	[Zip] [varchar](5) NULL,
	[County] [varchar](60) NULL,
	[Telephone] [varchar](15) NULL,
	[Latitude] Decimal(8,6) NULL,
	[Longitude] Decimal(9,6) NULL,
	[EnrollmentBeginDate] [date] NOT NULL,
	[RevalidationDate] [date] NOT NULL,
	[FileDate] [date] NOT NULL,
	[MedicallyFragileInd] [varchar](1) NULL,
	[ProvEmailID] [nvarchar](40) NULL,
	CONSTRAINT [PK_ProvMaster] PRIMARY KEY CLUSTERED 
(
	[ProvMasterKey] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY],
 CONSTRAINT [ProvMaster_NPI] UNIQUE NONCLUSTERED 
(
	[NPI] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO